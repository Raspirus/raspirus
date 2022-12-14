import pytest
import sys

from raspirus.backend.file_scanner_module import FileScanner

folder_scan_path = "files"
database_path = "../src/raspirus/backend/database/signatures.db"


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
    yield FileScanner(folder_scan_path, database_path)
