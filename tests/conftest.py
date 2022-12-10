import pytest
import sys

from raspirus.backend.file_scanner_module import FileScanner

folder_scan_path = "C:/Users/benbe/Documents/Coding/PyProjects/MaturaProject/tests/files"
bighash_path = "C:/Users/benbe/Documents/Coding/PyProjects/MaturaProject/src/raspirus/backend/BigHash.db"


@pytest.fixture
def capture_stdout(monkeypatch):
    buffer = {"stdout": "", "write_calls": 0}

    def fake_write(s):
        buffer["stdout"] += s
        buffer["write_calls"] += 1

    monkeypatch.setattr(sys.stdout, 'write', fake_write)
    return buffer


@pytest.fixture(scope="session")
def file_scanner():
    yield FileScanner(folder_scan_path, bighash_path)
