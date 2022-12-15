from raspirus.backend.file_scanner_module import FileScanner
from raspirus.backend.hash_api_module import HashAPI

db_location = "database/signatures.db"


# This function isn't finished yet and used for testing purposes, will then be implemented later
def main():
    print("#########################################################################")
    print("# Insert a directory to scan and the scanner will start automatically!  #")
    print("# Path syntax: 'C:/Users/someone/Documents/ToScan'                      #")
    print("# REMEMBER! No '/' at the end and no '                                  #")
    print("#########################################################################")
    path_to_check = str(input("Enter path: "))
    print("")
    fs = FileScanner(path_to_check, db_location)
    fs.start_scanner()


def updater():
    # Path to directory with md5 files/file extension, path to the bighash file
    hapi = HashAPI(db_location)
    hapi.update_db()
    print("Hash amount: " + hapi.count_hashes())


def more_info():
    hapi = HashAPI(db_location)
    hapi.get_hash_info("temp/test.json", "ecb9cf121345c404495d99c737c7d3bf")


main()
