from io import BytesIO
from .rs_api import encode, decode 
from pathlib import Path
from typing import Generator
__all__ = ["Controller"]
class Controller:
    CHUNK_SIZE = 4096
    # Example usage : 
    # >> Controller(Controller.from_csv("./bncsv-core/tests/data.csv")).to_csv()
    def __init__(self, data : Path |  bytes | str):
        if isinstance(data, str):
            data = Path(data)
        if isinstance(data, Path):
            self.input = lambda : Controller._chunk_read(open(data, "rb"), Controller.CHUNK_SIZE)
        if isinstance(data, bytes):
            self.input = lambda : data

    @staticmethod
    def _chunk_read(f, n) -> Generator[bytes, None, None]:
        # Helper to read a file in chunks of n bytes
        while (byte := f.read(n)):
            yield from byte

    def to_csv(self) -> bytes:
        # Convert the current bncsv object to csv utf-8 bytes
        data = BytesIO()
        decode(
            iter(self.input()),
            data
        )
        return data.getvalue()

    @staticmethod
    def from_csv(csv_path : Path | str | BytesIO | bytes) -> bytes:
        i_data = open(csv_path, "rb") if isinstance(csv_path, (Path,str)) else BytesIO(csv_path) if isinstance(csv_path, bytes) else csv_path
        o_data = BytesIO()
        encode(
            iter(Controller._chunk_read(i_data, Controller.CHUNK_SIZE)), 
            o_data
        )
        return o_data.getvalue()
