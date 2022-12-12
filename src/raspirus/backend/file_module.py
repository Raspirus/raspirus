"""Defines a file like module containing important information.

This module is used to store important properties of a file in a single class.
This allows us to concentrate on specific file properties,
without needing to save the whole file somehow.
"""

import hashlib
import os
import mmap


class File:
    """ File wrapper that contains important attributes of each file

    This class represents a file in the program and it not only saves the location
    of the file, but also its hash that gets generated automatically when you
    initialize this class.

    Attributes:
        name: Name of the file
        hash: Hash generated from content of the file
        path: Path to the file
    """
    name: str
    hash: bytes
    path: str

    def __init__(self, path):
        """Initializes the class using the given path

        It automatically starts the method to create the hash of the file
        and saves it to the variable hash

        Args:
            path: A string containing the path to the file

        Raises:
            IOError: File couldn't be found or accessed
        """
        if os.path.exists(path):
            self.path = path
            self.get_checksum()
            self.get_name()
        else:
            print(path)
            raise Exception("File not created, path invalid")

    def get_checksum(self, hash_factory=hashlib.md5, chunk_num_blocks=128):
        """ Generates the MD5 hash of the file

        It opens the file and reads its content to create a md5 hash from it.
        To create the hash, the hashlib library is used.
        Referencing -> https://stackoverflow.com/questions/1131220/get-md5-hash-of-big-files-in-python

        Arguments:
            hash_factory: Defines what type of hash we want, in this case md5
            chunk_num_blocks: Defines the amount chunks of the file it loads to memory at once.
                Especially important to prevent memory issues on small devices like the Raspberry Pi
        """
        hash_factory = hash_factory()
        with open(self.path, 'rb') as file_pointer:
            with mmap.mmap(file_pointer.fileno(), length=0, access=mmap.ACCESS_READ) as mmap_obj:
                while chunk := mmap_obj.read(chunk_num_blocks * hash_factory.block_size):
                    hash_factory.update(chunk)
        self.hash = hash_factory.digest()

    def get_name(self):
        """ Returns the name of the file as string """
        self.name = os.path.basename(self.path)
        return str(self.name)

    def get_hash(self):
        """ Returns the hash of the file as a string """
        return str(self.hash)
