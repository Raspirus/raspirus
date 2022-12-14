# The pytest library searches for files starting with test in the
# test directory, and then in those files, executes all functions
# starting with test
import pytest
import tracemalloc
from raspirus.backend.file_module import File
from raspirus.backend.file_scanner_module import FileScanner

tracemalloc.start()
single_file_path = "files/subfolder/dummy1.txt"


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


def test_single_file_scan(file_scanner):
    file_scanner.path = single_file_path
    file_scanner.start_scanner()
    print(f"RAM consumed: {tracemalloc.get_traced_memory()}")
    assert file_scanner.amount_of_files == 1


def test_folder_scan(file_scanner):
    snapshot1 = tracemalloc.take_snapshot()
    file_scanner.start_scanner()
    snapshot2 = tracemalloc.take_snapshot()
    top_stats = snapshot2.compare_to(snapshot1, 'lineno')
    for stat in top_stats[:10]:
        print(stat)
    assert file_scanner.amount_of_files >= 1


def test_print(capture_stdout):
    print("hello")
    tracemalloc.stop()
    assert capture_stdout["stdout"] == "hello\n"
