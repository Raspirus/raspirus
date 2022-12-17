""" This module compares hashes of a file to a given list of hashes

This module contains a class that can scan a file by using its hash and comparing it
to a bigger list of hashes. If the hash gets found, we consider the file as a virus,
else we consider it to be a clean file.
"""

import os.path
import time
import asyncio
import xxhash
import mmap
from raspirus.backend.database_api import HashAPI


class FileScanner:
    """ Defines the FileScanner object with all its functions and arguments.

    The FileScanner has the ability to compare a given file with a list of signatures and decide
    if the file is a virus or not. He does this using bisect and the hash of the file.

    Attributes:
        dirty_files: List of files whose hash was found in the signature list
    """
    amount_of_files = 0
    hasher: HashAPI
    dirty_files: list[str] = []
    path = ""

    def __init__(self, path, db_location):
        print("Scanner initialized")
        """ Initializes the class by setting the given parameters

        The class requires a location to collect all files from, it also collects files
        from subdirectories. And the class also needs the location of the signatures list,
        a list containing all known malicious file hashes.

        Args:
             path: Location of where you want to search for files, must be a directory

        Raises:
            IOError: If the path is not a directory, or could not be found

         """
        # Checks if path is a directory and sets it to the class
        if os.path.exists(path):
            self.path = path
            self.hasher = HashAPI(db_location)
        else:
            print(f"Path: dir ? {str(os.path.isdir(path))} & exists ?{str(os.path.exists(path))}")
            raise IOError("Invalid path or path not a directory")

    async def search_files(self, directory):
        for root, dirs, files in os.walk(directory):
            for file in files:
                self.amount_of_files += 1
                file_path = os.path.join(root, file)
                xxhash_hash = await asyncio.create_task(self.calculate_xxhash(file_path))
                if self.hasher.hash_exists(xxhash_hash):
                    print(f"{file_path}: {xxhash_hash}")
                    self.dirty_files.append(file_path)

    @staticmethod
    async def calculate_xxhash(file_path):
        if os.stat(file_path).st_size != 0:
            with open(file_path, 'rb') as f:
                with mmap.mmap(f.fileno(), 0, access=mmap.ACCESS_READ) as m:
                    return xxhash.xxh64(m).hexdigest()

    async def scan_files(self):
        if os.path.isdir(self.path):
            await self.search_files(self.path)
        else:
            self.amount_of_files += 1
            xxhash_hash = await asyncio.create_task(self.calculate_xxhash(self.path))
            if self.hasher.hash_exists(xxhash_hash):
                self.dirty_files.append(self.path)

    def start_scanner(self):
        print("Starting scanner...")
        tic = time.perf_counter()
        loop = asyncio.get_event_loop()
        loop.run_until_complete(self.scan_files())
        toc = time.perf_counter()
        print(
            (
                    (
                            (
                                    "\nScanner finished! \n"
                                    + f"Scanned files: {str(self.amount_of_files)} \n"
                                    + f"Bad files: {len(self.dirty_files)} \n"
                            )
                            + f"Scanned path: {self.path} \n"
                    )
                    + f"Execution time: {toc - tic:0.4f} seconds"
            )
        )
