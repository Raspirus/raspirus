import pytest
import sqlite3
from raspirus.backend.database_api import HashAPI

database_path = "../src/raspirus/backend/database/signatures.db"


def test_init():
    # Test that the class initializes correctly with a valid database location
    ha = HashAPI(database_path)
    assert ha.api_key is not None
    assert ha.db_connection is not None

    # Test that the class raises a sqlite3.Error with an invalid database location
    with pytest.raises(sqlite3.Error):
        ha = HashAPI("/invalid/path/to/database.db")


def test_init_table():
    # Test that the init_table method creates the table in the database
    ha = HashAPI(":memory:")
    ha.init_table()
    cur = ha.db_connection.cursor()
    cur.execute("SELECT name FROM sqlite_master WHERE type='table';")
    assert cur.fetchone()[0] == "signatures"


def test_insert_hash():
    # Test that the insert_hash method correctly inserts a hash into the database
    ha = HashAPI(":memory:")
    ha.init_table()
    ha.insert_hash("1234567890abcdef1234567890abcdef", "00001")
    cur = ha.db_connection.cursor()
    cur.execute("SELECT * FROM signatures;")
    assert cur.fetchone() == ("1234567890abcdef1234567890abcdef", "00001")


def test_insert_hashes():
    # Test that the insert_hashes method correctly inserts multiple hashes into the database
    ha = HashAPI(":memory:")
    ha.init_table()
    ha.insert_hashes([("1234567890abcdef1234567890abcdef", "00001"),
                      ("abcdef1234567890abcdef1234567890", "00002")])
    cur = ha.db_connection.cursor()
    cur.execute("SELECT * FROM signatures;")
    assert cur.fetchall() == [("1234567890abcdef1234567890abcdef", "00001"),
                              ("abcdef1234567890abcdef1234567890", "00002")]


def test_hash_exists():
    # Test that the hash_exists method returns True for a hash that exists in the database
    ha = HashAPI(":memory:")
    ha.init_table()
    ha.insert_hash("1234567890abcdef1234567890abcdef", "00001")
    assert ha.hash_exists("1234567890abcdef1234567890abcdef")


# Test get_latest_file_nr method
def test_get_latest_file_nr(mocker):
    # Create a mock cursor
    mock_cursor = mocker.Mock()
    # Set the expected return value for the cursor's execute method
    mock_cursor.execute.return_value = [("00005",)]
    # Create a mock connection
    mock_conn = mocker.Mock()
    # Set the expected return value for the connection's cursor method
    mock_conn.cursor.return_value = mock_cursor

    # Initialize the HashAPI object with the mock connection
    api = HashAPI(mock_conn)

    # Ensure the get_latest_file_nr method returns the correct value
    assert api.get_latest_file_nr() == "00005"


# Test count_hashes method
def test_count_hashes(mocker):
    # Create a mock cursor
    mock_cursor = mocker.Mock()
    # Set the expected return value for the cursor's execute method
    mock_cursor.execute.return_value = [("1000",)]
    # Create a mock connection
    mock_conn = mocker.Mock()
    # Set the expected return value for the connection's cursor method
    mock_conn.cursor.return_value = mock_cursor

    # Initialize the HashAPI object with the mock connection
    api = HashAPI(mock_conn)

    # Ensure the count_hashes method returns the correct value
    assert api.count_hashes() == "1000"


# Test remove_hash method
def test_remove_hash(mocker):
    # Create a mock cursor
    mock_cursor = mocker.Mock()
    # Create a mock connection
    mock_conn = mocker.Mock()
    # Set the expected return value for the connection's cursor method
    mock_conn.cursor.return_value = mock_cursor

    # Initialize the HashAPI object with the mock connection
    api = HashAPI(mock_conn)

    # Ensure the remove_hash method executes the correct SQL query
    api.remove_hash("hash_value")
    mock_cursor.execute.assert_called_with(" DELETE FROM signatures WHERE hash = ? ", ("hash_value",))


# Test download_files method
def test_download_files(mocker):
    # Create a mock connection
    mock_conn = mocker.Mock()

    # Initialize the HashAPI object with the mock connection
    api = HashAPI(mock_conn)

    # Set the expected return value for the db_is_updated method
    api.db_is_updated.return_value = False
