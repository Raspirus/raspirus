# The pytest library searches for files starting with test in the
# test directory, and then in those files, executes all functions
# starting with test
import pytest
from raspirus.backend.file_module import File
from raspirus.backend.file_scanner_module import FileScanner

single_file_path = "C:/Users/benbe/Documents/Coding/PyProjects/MaturaProject/tests/files/subfolder/dummy1.txt"


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


def test_single_file_scan_init(file_scanner):
    file_scanner.path = single_file_path
    file_scanner.initialize_scanner()
    assert len(file_scanner.unscanned_list) == 1


def test_folder_scan_init(file_scanner):
    file_scanner.initialize_scanner()
    assert len(file_scanner.unscanned_list) is not None


def test_single_file_scan_start_error(file_scanner):
    file_scanner.path = single_file_path
    with pytest.raises(Exception):
        file_scanner.start_scanner()


def test_single_file_scan_start_success(file_scanner):
    file_scanner.path = single_file_path
    file_scanner.initialize_scanner()
    file_scanner.start_scanner()
    assert len(file_scanner.clean_files) == 1
    assert len(file_scanner.dirty_files) == 0


def test_folder_scan_start_error(file_scanner):
    with pytest.raises(Exception):
        file_scanner.start_scanner()


def test_folder_scan_start_success(file_scanner):
    file_scanner.initialize_scanner()
    file_scanner.start_scanner()
    assert len(file_scanner.clean_files) > 0
    assert len(file_scanner.dirty_files) == 0


def test_print(capture_stdout):
    print("hello")
    assert capture_stdout["stdout"] == "hello\n"
