import binascii
from pathlib import Path
from typing import Generator


def crc32_hash(data):
    """
    Calculate the CRC-32 hash of the given data.

    Parameters:
    data (str): The input string to be hashed.

    Returns:
    int: The CRC-32 hash value.
    """
    if isinstance(data, str):
        # Convert the string to bytes before calculating the CRC-32 hash
        data = data.encode("utf-8")
    elif not isinstance(data, bytes):
        raise TypeError("Input data must be a string or bytes.")

    # Calculate CRC-32 hash
    crc = binascii.crc32(data)

    # Ensure non-negative result
    return crc & 0xFFFFFFFF


def read_file_bytes(file_path: Path) -> bytes:
    """Returns the stories15M.bin file as a bytes array"""
    file_bytes = b""
    try:
        with open(file_path, "rb") as file:
            file_bytes = file.read()

    except FileNotFoundError:
        print(f"Unable to open the file {file_path}!")

    return file_bytes


def generate_chunks(data: bytes, chunk_size: int) -> Generator[bytes, None, None]:
    """Generator function to iterate over chunks"""
    for i in range(0, len(data), chunk_size):
        yield data[i : i + chunk_size]
