from typing import Iterator
from pathlib import Path
from io import BytesIO, StringIO
def encode(input: Iterator[bytes[1], None, None], writer : BytesIO) -> None:
    pass
       
def decode(input: Iterator[bytes[1], None, None], writer : StringIO) -> None:
    pass

class Controller:
    CHUNK_SIZE = 4096
    # Example usage : 
    # >> BnCsvController(BnCsvController.from_csv("./bncsv-core/tests/data.csv")).to_csv()
    def __init__(self, data : Path | bytes | str):
        pass

    def to_csv(self) -> bytes:
        pass

    @staticmethod
    def from_csv(csv_path : Path | str | BytesIO | bytes) -> bytes:
        pass
