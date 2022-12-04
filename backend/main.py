from Raspirus.backend.file_scanner_module import FileScanner
from Raspirus.backend.hash_api_module import HashAPI

signature_path = "BigHash.db"
signature_lists_path = "backend/SignatureLists/*md5"


# This function isn't finished yet and used for testing purposes, will then be implemented later
def main():
    print("#########################################################################")
    print("# Insert a directory to scan and the scanner will start automatically!  #")
    print("# Path syntax: 'C:/Users/someone/Documents/ToScan'                      #")
    print("# REMEMBER! No '/' at the end and no '                                  #")
    print("#########################################################################")
    path_to_check = str(input("Enter path: "))
    print("")
    fs = FileScanner(path_to_check, signature_path)
    fs.start_scanner()


def updater():
    # Path to directory with md5 files/file extension, path to the bighash file
    # 38797306 Hashes
    hapi = HashAPI(signature_lists_path, signature_path)
    hapi.update_bighash()
    h_list = hapi.get_hash()
    print("Length: " + str(len(h_list)))


def downloader():
    hapi = HashAPI(signature_lists_path, signature_path)
    hapi.download_new_signatures("backend/SignatureLists")


def more_info():
    hapi = HashAPI(signature_lists_path, signature_path)
    hapi.get_hash_info("backend/temp/test.json", "ecb9cf121345c404495d99c737c7d3bf")


main()
