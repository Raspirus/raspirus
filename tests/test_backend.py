# The pytest library searches for files starting with test in the
# test directory, and then in those files, executes all functions
# starting with test
import pytest
import tracemalloc
from raspirus.backend.file_module import File
from raspirus.backend.file_scanner_module import FileScanner

single_file_path = "files/subfolder/dummy1.txt"
folder_scan_path = "files"
database_path = "../src/raspirus/backend/database/signatures.db"


def test_file_creation_error():
    with pytest.raises(Exception):
        File(path="")


def test_file_creation_success():
    file = File(path=single_file_path)
    assert file.name == "dummy1.txt"
    assert file.hash is not None
    assert file.hash is not None


def test_file_scanner_creation():
    with pytest.raises(Exception):
        FileScanner("", "")
    with pytest.raises(Exception):
        FileScanner(None, None)


def test_folder_scan():
    tracemalloc.start()
    snapshot1 = tracemalloc.take_snapshot()
    file_scanner = FileScanner(path=folder_scan_path, db_location=database_path)
    file_scanner.start_scanner()
    snapshot2 = tracemalloc.take_snapshot()
    top_stats = snapshot2.compare_to(snapshot1, 'lineno')
    tracemalloc.stop()
    for stat in top_stats[:10]:
        print(stat)
    assert file_scanner.amount_of_files >= 1


def test_single_file_scan():
    file_scanner = FileScanner(path=single_file_path, db_location=database_path)
    file_scanner.start_scanner()
    assert file_scanner.amount_of_files == 1


def test_print(capture_stdout):
    print("hello")
    assert capture_stdout["stdout"] == "hello\n"
