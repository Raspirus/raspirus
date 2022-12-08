""" A module that contains functions to control the Hash API.

This module keeps the signature list up-to-date and clean.
The update is currently triggered manually, but on each update,
the doubles get removed from the list and the list gets sorted.

References -> https://virusshare.com
"""

import os
import shutil
import glob
from urllib.error import HTTPError
import wget
from dotenv import load_dotenv

class HashAPI:
    """
    This class will do the following tasks using the Virusshare API
        - Periodically check if new hash signatures are available
        - Remove Hashes that are found twice in files
        - If needed, find more specific data on a Hash
        - Update the Hash signatures

    Attributes:
        api_key: The API key used to access VirusShare
        bighash_path: Path to the file containing all signatures in hash format
        signature_list_path: Folder containing all signature lists as individual files
        hash_list: List of hashes extracted from the bighash_file
        file_list: List of files, basically the files mentioned in the signature_list_path
    """
    load_dotenv() # Loads environment variables
    api_key = os.getenv('VIRUS_API')
    bighash_path = ""
    signature_list_path = ""
    hash_list = []
    file_list = []

    def __init__(self, signature_path, bighash_path):
        """ Initializes the class setting the given parameters

        Arguments:
            signature_path -> Path to the folder containing all hash signatures
            bighash_path -> Path to the one big file containing all hashes merged together

        """
        self.signature_list_path = signature_path
        self.bighash_path = bighash_path
        self.file_list = glob.glob(self.signature_list_path)

    def get_hash(self):
        """ Returns the hash list """
        return self.hash_list

    # A function to merge all hash lists into one
    def merge_files(self):
        """ Merges all hashes of files into one big file """
        print("Starting file merging")
        with open(self.bighash_path, 'wb', encoding="utf8") as wfd:
            for file in self.file_list:
                with open(file, 'rb') as file_pointer:
                    shutil.copyfileobj(file_pointer, wfd)

    def refactor_bighash(self):
        """ Removed doubles and incorrect hashes from the big file """
        # Extract all hashes from file and put them into an array
        # then overwrite the file with this array and put a message at the end of the file
        # The message will then specify the last merged file
        # This is useful to later detect which file still needs to be added

        # Read all hashes from file
        print("Reading all hashes from file...")
        with open(self.bighash_path, encoding="utf8") as file_pointer:
            for line in file_pointer:
                # Comments in the file need to be removed
                if not line.startswith("#"):
                    self.hash_list.append(str(line))

        # Here we convert the list to a set and back, this removes duplicates
        self.hash_list = list(set(self.hash_list))
        self.hash_list.sort()

        # Write all hashes to file
        print("Writing all hashes to file...")
        with open(self.bighash_path, 'w', encoding="utf8") as file_pointer:
            for hashes in self.hash_list:
                file_pointer.writelines(hashes)
            # Write last line to identify which Hash we included for last
            file_pointer.writelines("# Last added: " + self.file_list[len(self.file_list) - 1])

    def bighash_is_updated(self):
        """ Checks the last line in the big file to determine if its updated
        If the name of the file mentioned in the last line of bighash
        is the same as the last item in file_list
        we consider the file as updated
         """
        print("Checking if file is updated")
        if os.path.exists(self.bighash_path):
            with open(self.bighash_path, 'rb', encoding="utf8") as file_pointer:
                try:  # catch OSError in case of a one line file
                    file_pointer.seek(-2, os.SEEK_END)
                    while file_pointer.read(1) != b'\n':
                        file_pointer.seek(-2, os.SEEK_CUR)
                except OSError:
                    file_pointer.seek(0)
                last_line = file_pointer.readline().decode()
                print(last_line)
                print(last_line.split("added:", 1)[1])
                print(self.file_list[len(self.file_list) - 1])
                return (last_line.split("added:", 1)[1]).strip() == \
                       (self.file_list[len(self.file_list) - 1]).strip()
        return False

    def update_bighash(self):
        """ Updates the big file if it hasn't already"""
        # First check if the file exists, then if the latest hash list has been added
        if self.bighash_is_updated():
            print("File up to date, nothing to do")
        else:
            print("File not up to date, syncing")
            self.file_list = glob.glob(self.signature_list_path)
            self.merge_files()
            self.refactor_bighash()

    def download_new_signatures(self, download_path):
        """ Downloads new hash signatures from the online database
        and puts them in a folder

        Arguments:
            download_path -> Where to save the downloaded files

        """
        # Downloads new hashes from Virusshare if available
        # Use the last downloaded Hash to create a URL and add +1 to it
        # If it exists we download it and add +1 again
        # We do this until a 404 error arises and then we stop
        last_sign = self.file_list[len(self.file_list) - 1]
        last_sign = last_sign.split("VirusShare_", 1)[1]
        last_sign_int = int(last_sign[:-4])

        # Now we increase the number and try to download the resource
        while True:
            try:
                last_sign_int += 1
                filename = "VirusShare_00" + str(last_sign_int) + ".md5"
                url = "https://virusshare.com/hashfiles/" + filename
                wget.download(url, download_path + "/" + filename)
                print("New file downloaded: " + filename)
            except HTTPError as err:
                if err.code == 404:
                    print("No more file to download")
                    break
                print("ERROR: " + str(err))
                break

    def get_hash_info(self, json_location, virus_hash):
        """ Creates a JSON file containing information about a given hash

        Arguments:
            json_location -> Where to save the generated JSON file
            virus_hash -> The hash to lookup information about

        """

        # Retrieves more detailed information about a specific hash by using the Virusshare API
        url = "https://virusshare.com/apiv2/file?apikey=" + self.api_key + "&hash=" + virus_hash

        try:
            wget.download(url, json_location)
            with open(json_location, encoding="utf8") as file_pointer:
                for line in file_pointer:
                    print(line)
        except HTTPError as err:
            if err.code == 243:
                print("Limit exceeded")
            elif err.code == 404:
                print("Request not found")
            else:
                print("Error: " + str(err))
