""" This module compares hashes of a file to a given list of hashes

This module contains a class that can scan a file by using its hash and comparing it
to a bigger list of hashes. If the hash gets found, we consider the file as a virus,
else we consider it to be a clean file.
"""

import os.path
from raspirus.backend.file_module import File
from raspirus.backend.hash_api_module import HashAPI


class FileScanner:
    """ Defines the FileScanner object with all its functions and arguments.

    The FileScanner has the ability to compare a given file with a list of signatures and decide
    if the file is a virus or not. He does this using bisect and the hash of the file.

    Attributes:
        dirty_files: List of files whose hash was found in the signature list
    """
    amount_of_files = 0
    hasher: HashAPI
    dirty_files: list[File] = []
    path = ""

    def __init__(self, path, db_location):
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
        if os.path.isdir(path):
            self.path = path
            self.hasher = HashAPI(db_location)
        else:
            print("Path: dir ? " + str(os.path.isdir(path)) + " & exists ?" + str(os.path.exists(path)))
            raise Exception("Invalid path or path not a directory")

    def start_scanner(self):
        if os.path.isdir(self.path):
            for path, directories, file_names in os.walk(self.path):
                for file_name in file_names:
                    file_path = path + "/" + file_name
                    file = File(file_path)
                    self.amount_of_files += 1
                    if self.hasher.hash_exists(file.get_hash()):
                        self.dirty_files.append(file)
        else:
            file = File(self.path)
            self.amount_of_files += 1
            if self.hasher.hash_exists(file.get_hash()):
                self.dirty_files.append(file)

        print("\nScanner finished! \n" +
              "Scanned files: " + str(self.amount_of_files) + "\n" +
              "Bad files: " + str(len(self.dirty_files)) + "\n" +
              "Scanned path: " + self.path)
