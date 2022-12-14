""" A module that contains functions to control the Hash API.

This module keeps the signature list up-to-date and clean.
The update is currently triggered manually, but on each update,
the doubles get removed from the list and the list gets sorted.

References -> https://virusshare.com
"""

import os
import time
from urllib.request import urlopen
import sqlite3
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
    """
    load_dotenv()  # Loads environment variables
    api_key = os.getenv('VIRUS_API')
    db_connection = None

    def __init__(self, db_location):
        """ Initializes the class setting the given parameters

        It creates an object that can be used to interact with the Virusshare API
        and the signatures' database.
        """

        try:
            self.db_connection = sqlite3.connect(db_location)
            self.init_table()
        except sqlite3.Error as e:
            raise Exception("Connection to DB failed: " + str(e))

    def init_table(self):
        sql = ''' CREATE TABLE IF NOT EXISTS signatures (
                    hash varchar(32) PRIMARY KEY,
                    file_nr varchar(5)
                    ); '''
        try:
            cur = self.db_connection.cursor()
            cur.execute(sql)
            self.db_connection.commit()
        except sqlite3.Error as e:
            print("SQL table not created: " + str(e))

    def insert_hash(self, hash_str, file_nr):
        sql = ''' INSERT INTO signatures(hash, file_nr)
              VALUES (?, ?) '''

        try:
            cur = self.db_connection.cursor()
            cur.execute(sql, (hash_str, file_nr))
            self.db_connection.commit()
        except sqlite3.Error as e:
            print("Hash (" + hash_str + ") not inserted: " + str(e))

    def insert_hashes(self, hashes):
        sql = ''' INSERT INTO signatures(hash, file_nr)
              VALUES (?, ?) '''

        try:
            cur = self.db_connection.cursor()
            cur.executemany(sql, hashes)
            self.db_connection.commit()
        except sqlite3.Error as e:
            print("Hashes not inserted: " + str(e))

    def hash_exists(self, hash_str):
        sql = ''' SELECT hash FROM signatures
                WHERE hash = ? '''

        cur = self.db_connection.cursor()
        cur.execute(sql, (hash_str,))
        rows = cur.fetchone()

        if rows:
            return True
        return False

    def get_latest_file_nr(self):
        sql = ''' SELECT file_nr
                    FROM signatures
                    ORDER BY file_nr DESC
                    LIMIT 1; '''

        cur = self.db_connection.cursor()
        cur.execute(sql)

        try:
            return ''.join(map(str, cur.fetchone()))
        except Exception:
            return 'None'

    def count_hashes(self):
        sql = ''' SELECT COUNT(hash)
                    FROM signatures '''

        cur = self.db_connection.cursor()
        cur.execute(sql)

        return ''.join(map(str, cur.fetchone()))

    def update_db(self):
        big_tic = time.perf_counter()
        if not self.db_is_updated():
            file_nr = self.get_latest_file_nr()
            if file_nr == 'None':
                file_nr = "00000"

            while True:
                try:
                    tic = time.perf_counter()
                    # Format the correct filename for the URL
                    filename = "VirusShare_" + file_nr + ".md5"
                    # Extract the file online
                    url = "https://virusshare.com/hashfiles/" + filename
                    file = urlopen(url)
                    # Read each line and add it to the database
                    hashes = []
                    for line in file:
                        line_n = str(line).replace("b'", "").replace("\\n'", "")
                        if not line_n.startswith("#"):
                            hashes.append((line_n, file_nr))
                    self.insert_hashes(hashes)
                    toc = time.perf_counter()
                    print(f"Downloaded {filename} in {toc - tic:0.4f} seconds")
                    file_nr = int(file_nr) + 1
                    file_nr = f'{file_nr:05d}'
                except HTTPError as err:
                    if err.code == 404:
                        print("No more files to download")
                        break
                    print("ERROR: " + str(err))
                    break
        else:
            print("DB already up-to-date")
        big_toc = time.perf_counter()
        print(f"Executed in {big_toc - big_tic:0.4f} seconds")

    def db_is_updated(self):
        """ Checks if the Database is up-to-date.

        It uses another function to retrieve the latest file_nr in the database.
        Then using that, it tries to increase it and reach the file with the new number.
        If the request is successful, it means the database is outdated, else its updated

        Returns:
            False - Database is NOT updated
            True - Database is updated

        """
        file_nr = self.get_latest_file_nr()
        if file_nr == 'None':
            return False

        try:
            file_nr = int(file_nr) + 1
            # A method to add leading zeros
            file_nr = f'{file_nr:05d}'
            filename = "VirusShare_" + file_nr + ".md5"
            url = "https://virusshare.com/hashfiles/" + filename
            # Will generate an error if url is unreachable
            urlopen(url)
            return False
        except HTTPError as err:
            if err.code == 404:
                return True
            print("Error! " + str(err))

    def get_hash_info(self, json_location, virus_hash: str):
        """ Creates a JSON file containing information about a given hash

        This actually uses the API key and is a purely optional function.
        Through an API call, we can get much more information about a hash.
        The hash has to be detected as virus, else it won't work.

        Args:
            json_location: Where to save the generated JSON file
            virus_hash: The hash to lookup information about

        Raises:
            APIError: The API call produced an error, for example the key used is invalid
        """

        # Retrieves more detailed information about a specific hash by using the Virusshare API
        url = "https://virusshare.com/apiv2/file?apikey=" + str(self.api_key) + "&hash=" + str(virus_hash)

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
                raise Exception("Error: " + str(err))
