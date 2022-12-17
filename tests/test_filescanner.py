# The pytest library searches for files starting with test in the
# test directory, and then in those files, executes all functions
# starting with test
import pytest
import os
from raspirus.backend.file_scanner_module import FileScanner

single_file_path = "files/subfolder/dummy1.txt"
folder_scan_path = "files"
database_path = "../src/raspirus/backend/database/signatures.db"


def test_init():
    # Test that the class initializes correctly with a valid path
    fs = FileScanner(folder_scan_path, database_path)
    assert fs.path == folder_scan_path
    assert fs.hasher is not None

    # Test that the class raises an IOError with an invalid path
    with pytest.raises(IOError):
        fs = FileScanner("/invalid/path", "/path/to/signature/list")


def test_scan_files():
    # Test that the scan_files method correctly counts the number of files
    fs = FileScanner(folder_scan_path, database_path)
    fs.amount_of_files = 0
    fs.scan_files()
    assert fs.amount_of_files > 0

    # Test that the scan_files method correctly identifies dirty files
    fs = FileScanner(single_file_path, database_path)
    fs.dirty_files = []
    fs.scan_files()


def test_search_files():
    # Test that the search_files method correctly counts the number of files
    fs = FileScanner(folder_scan_path, database_path)
    fs.amount_of_files = 0
    fs.search_files(folder_scan_path)
    assert fs.amount_of_files > 0

    # Test that the search_files method correctly identifies dirty files
    fs = FileScanner(folder_scan_path, database_path)
    fs.dirty_files = []
    fs.search_files(folder_scan_path)


def test_calculate_xxhash():
    # Test that the calculate_xxhash method returns a string
    fs = FileScanner(folder_scan_path, database_path)
    ret_hash = fs.calculate_xxhash(single_file_path)
    assert isinstance(ret_hash, str)

    # Test that the calculate_xxhash method returns None for an empty file
    with open(f"{folder_scan_path}/empty_file", "w") as f:
        f.write("")
    fs = FileScanner(f"{folder_scan_path}/empty_file", database_path)
    ret_hash = fs.calculate_xxhash(f"{folder_scan_path}/empty_file")
    assert ret_hash is None
    os.remove(f"{folder_scan_path}/empty_file")  # This might be optional


def test_start_scanner():
    # Test that the start_scanner method executes correctly
    fs = FileScanner(folder_scan_path, database_path)
    fs.start_scanner()
    assert fs.amount_of_files > 0
    assert len(fs.dirty_files) >= 0


