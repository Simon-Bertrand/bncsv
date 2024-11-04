import bncsv_py

class TestPyConvert:
    def test_py_encode_decode(self):
        csv_bytes = b'42.91,46.02,87.53\n65.55,31.57,3.79\n28.15,42.25,61.99\n13.86,22.85,94.43\n'
        gt_compressed = b'\x9eQFq?\x1b\xf45\x9c\x08\x03\xb4\x87\x9d_\x0c\xf5\xaa\x06yp9J!\x9a\xd5x{\x95\x83\x12\xa7Y`'
        assert bncsv_py.Controller.from_csv(csv_bytes) == gt_compressed
        assert bncsv_py.Controller(gt_compressed).to_csv() == csv_bytes