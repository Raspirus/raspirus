# WARNING !
# For tests purposes only!

import time
from urllib.request import urlopen
import sqlite3
from urllib.error import HTTPError
import threading


class Hasher:
    db_location = "signatures.db"

    def insert_hash(self, hash_str, file_nr):
        db_connection = sqlite3.connect(self.db_location)

        sql = ''' INSERT INTO signatures(hash, file_nr)
              VALUES (?, ?) '''

        try:
            cur = db_connection.cursor()
            cur.execute(sql, (hash_str, file_nr))
            db_connection.commit()
        except sqlite3.Error as e:
            if str(e) != "UNIQUE constraint failed: signatures.hash":
                print("Hash (" + hash_str + ") not inserted: " + str(e))

        db_connection.close()

    def get_latest_file_nr(self):
        db_connection = sqlite3.connect(self.db_location)

        sql = ''' SELECT file_nr 
                    FROM signatures
                    ORDER BY file_nr DESC
                    LIMIT 1; '''

        cur = db_connection.cursor()
        cur.execute(sql)

        try:
            return ''.join(map(str, cur.fetchone()))
        except Exception:
            return 'None'

    def insert_file(self, file_nr):
        tic = time.perf_counter()
        # Format the correct filename for the URL
        filename = "VirusShare_" + file_nr + ".md5"
        print(threading.currentThread().getName() + ": Work started on " + filename)
        # Extract the file online
        url = "https://virusshare.com/hashfiles/" + filename
        url_file = urlopen(url)

        for line in url_file:
            line_n = str(line).replace("b'", "").replace("\\n'", "")
            if not line_n.startswith("#"):
                self.insert_hash(line_n, file_nr)
        toc = time.perf_counter()
        print(f"Downloaded {filename} in {toc - tic:0.2f} seconds")

    def update_db(self):
        # file_nr = self.get_latest_file_nr()
        file_nr = "00000"
        print("File Nr: " + str(file_nr))
        if file_nr == 'None':
            file_nr = "00000"

        while True:
            try:
                t1 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t2 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t3 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t4 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t5 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t6 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t7 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t8 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t9 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t10 = threading.Thread(target=self.insert_file, args=(file_nr,))
                file_nr = int(file_nr) + 1
                file_nr = f'{file_nr:05d}'

                t1.start()
                t2.start()
                t3.start()
                t4.start()
                t5.start()
                t6.start()
                t7.start()
                t8.start()
                t9.start()
                t10.start()

                t1.join()
                t2.join()
                t3.join()
                t4.join()
                t5.join()
                t6.join()
                t7.join()
                t8.join()
                t9.join()
                t10.join()

            except HTTPError as err:
                if err.code == 404:
                    print("No more files to download")
                    break
                print("ERROR: " + str(err))
                break


if __name__ == '__main__':
    hasher = Hasher()
    tic_time = time.perf_counter()
    hasher.update_db()
    toc_time = time.perf_counter()
    print(f"General time:  {toc_time - tic_time:0.2f} seconds")
