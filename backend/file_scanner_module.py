""" This module compares hashes of a file to a given list of hashes

This module contains a class that can scan a file by using its hash and comparing it
to a bigger list of hashes. If the hash gets found, we consider the file as a virus,
else we consider it to be a clean file.

"""

import os.path
from bisect import bisect_left
from Raspirus.backend.file_module import File


def bi_contains(lst, item):
    """ Uses the bisec module to search for an item in the list efficiently

    It uses the well-known bisect algorithm to search for a given item in a given list.
    In our case we use it to search for the hash of the file in the signature list.
    For this to work, the list has to be sorted first
    Referencing -> https://stackoverflow.com/questions/2701173/most-efficient-way-for-a-lookup-search-in-a-huge-list-python
    A short description:
        If item is larger than the last it's not in the list, but the bisect would
        find `len(lst)` as the index to insert, so check that first. Else, if the
        item is in the list then it has to be at index bisect_left(lst, item)

    Args:
        lst: List of items where you want to search for something
        item: Item you want to search in the list

    Returns:
        Recursively calls this function until either the whole list has been searched,
        or the item has been found. If the item has been found, return true, else false
    """
    return (item <= lst[-1]) and (lst[bisect_left(lst, item)] == item)


def print_star(counter):
    if counter % 10 == 0:
        print("*")
        print(str(counter) + ".", end=" ")
    else:
        print("*", end=" ")


class FileScanner:
    """ Defines the FileScanner object with all its functions and arguments.

    The FileScanner has the ability to compare a given file with a list of signatures and decide
    if the file is a virus or not. He does this using bisect and the hash of the file.

    Attributes:
        unscanned_list: List containing all files found in the specified path
        clean_files: List of files whose hash was not found in the signature list
        dirty_files: List of files whose hash was found in the signature list
        hash_list: List of all hashes from the signature list

    """
    unscanned_list = []
    clean_files = []
    dirty_files = []
    hash_list = []
    path = ""
    signature_db_path = ""

    # Update status holders
    scanned_progress = 0

    def __init__(self, path, signature_path):
        """ Initializes the class by setting the given parameters

        The class requires a location to collect all files from, it also collects files
        from subdirectories. And the class also needs the location of the signatures list,
        a list containing all known malicious file hashes.

        Args:
             path: Location of where you want to search for files, must be a directory
             signature_path: Location of the file containing all virus hashes

        Raises:
            IOError: If the path is not a directory, or could not be found

         """
        # Checks if path is a directory and sets it to the class
        if os.path.isdir(path) and os.path.isfile(signature_path):
            self.path = path
            self.signature_db_path = signature_path
        else:
            print("Path: " + str(os.path.isdir(path)) + " & " + str(os.path.exists(path)))
            print("SignaturePath: " + str(os.path.isdir(signature_path)) +
                  " & " + str(os.path.exists(signature_path)))
            raise Exception("Invalid path or path not a directory")

    def get_hash_list(self):
        """ Creates a list of hashes, extracted from a file

        Tries to open the file containing all hashes and read it line by line
        Each line is then added to the hash_list
        This might cause errors on smaller devices, because if the list gets very big,
        and it all needs to be saved to memory, a smaller device might not be able to
        save it and raise an MemoryOutOfBound Exception, or crash entirely!

        Raises:
            IOError: If the given location of the signatures list can't be found
            or opened / accessed
        """
        try:
            with open(self.signature_db_path, encoding="utf8") as file_pointer:
                for line in file_pointer:
                    # Comments in the file need to be removed
                    if not line.startswith("#"):
                        self.hash_list.append(str(line))
        except FileNotFoundError as error:
            print("Error while reading the SignatureDB occured: " + str(error))

    def get_file_list(self):
        """ Creates a list of File objects

        Finds all files in a specified path and adds them to the unscanned_list
        """
        for path, directories, file_names in os.walk(self.path):
            # print("Directories found: " + str(directories))
            for file_name in file_names:
                file_path = path + "/" + file_name
                file = File(file_path)
                self.unscanned_list.append(file)

    def compare_lists(self):
        """ Uses bisect to compare each hash of a File to any Hash in the hash_list

        Compares each hash of a file with the hashes in the signature list
        If it finds something, the file is added to the dirty_files list, else
        it is added to the clean_files list
        """
        counter = 1
        self.hash_list.sort()
        for file in self.unscanned_list:
            if bi_contains(self.hash_list, file.get_hash()):
                self.dirty_files.append(file)
            else:
                self.clean_files.append(file)
            self.scanned_progress = counter
            counter += 1

    ################################################################
    # These are the functions to start and initialize the scanner:
    ###############################################################

    def initialize_scanner(self):
        """ Initializes the list of files and hashes.

        This action is useful to later start the scanner. It initializes the used lists,
        but does not start scanner! It is clumsy, but necessary for the frontend attached
        to this application. Because after initializing, the user could modify the lists,
        which are now filled, and then start the scanner later.
        """

        self.get_file_list()
        print("File list created! " + str(len(self.unscanned_list)) +
              " files found in " + self.path)
        self.get_hash_list()
        print("Hash list created! " + str(len(self.hash_list)) + " Hashes found")


    def start_scanner(self):
        """ Starts the scanner.

        This simply starts the comparison of the file hash with the signatures list.
        To be effective at it, it is necessary to first call the function initialize_scanner()
        to fill the necessary list.

        Raises:
            OrderError: If the list have not been initialized first
        """
        if (self.unscanned_list is None) or (self.hash_list is None):
            raise Exception("Call initialize_scanner() first!")

        self.compare_lists()
        print("Scanner finished! \n"
              "Scanned files: " + str(len(self.unscanned_list)) + "\n"
              "Good files: " + str(len(self.clean_files)) + "\n"
              "Bad files: " + str(len(self.dirty_files)) + "\n"
              "Scanned path: " + self.path)
