# BinaryNumericalCSV
A binary file format inspired by CSV files to fast compress and decompress numeric data.
## Introduction
#### Do you have huge numerical CSV files with high precision ?
#### Do you want to compress them on the fly in a fast and efficient way ?

The BinaryNumericalCSV is a Rust project that aims to provide a solution for compressing numerical data (essentially floating numbers) in binary CSV format which allows an infinite precision storage. This project addresses the need for efficient storage and transmission of large datasets without compromising on accuracy.

## Benefits

- Compression Rate >2 (using Huffman's method)
- Infinite precision storage of decimal numbers
- Fast & memory efficient 
- Small executable
- CLI tool allowing directly to convert data from a shell using glob patterns or stdin/stdout.
- Multithreading per file when using a glob pattern or multiple inputs
- Python binding

## Installation
Git, Python>3.10 and Cargo are needed to be available in the Terminal.

#### Clone the project
```bash
git clone https://github.com/Simon-Bertrand/BinaryNumericalCSV bncsv-repo
cd bncsv-repo
```
#### Build both projects
```bash
source ./cmds/build-env.sh && ./cmds/build.sh
cd build
```
The Python bound module, in the form of a wheel package, is located in the folder `bncsv_py`.
The CLI executable and the compiled Python package are located in the `bncsv-core` folder.

You can then create an executable's symlink or temporary add the `./build/bncsv-core` folder in the PATH.
#### Build separate projects
To build only the CLI, run:
```bash
cargo build --bin bncsv --release --features cli,multithreading 
```
To build only the Python package, run:
```bash
cargo build -p bncsv_py --release
```


## Python binding usage
The crate provides a Python library that call the Rust conversion core functions under the hood. To install the library in your currently activated env : 
```bash
pip install setuptools-rust ./bindings/py/
```
Once installed, in Python console : 
```python
import bncsv_py
bncsv_data = bncsv_py.Controller.from_csv("./data.csv")
bncsv_py.Controller(bncsv_data).to_csv()
>> b'0.09576473636221827,...'
```


## CLI Usage


Let's start by showing the help command with : 

```bash
bncsv # The same as 'bncsv --help'
```
```
>> Usage: bncsv.exe [<paths...>] -i <input-type> [-o <output>] [--abs-pathbase <abs-pathbase>] [-p] [-j <jobs>]
BNCSV Format CLI Tool

Positional Arguments:
  paths             input file glob paths

Options:
  -i, --input-type  type of input file : ['csv', 'bncsv']
  -o, --output      output path dir
  --abs-pathbase    path base for absolute glob input paths
  -p, --pipe        use stdin as input
  -j, --jobs        number of jobs to run in parallel
  --help            display usage information
```

To encode a CSV file use : 
```bash
bncsv myCsv.csv -i csv > out.bncsv
# Or directly name the output
bncsv myCsv.csv -i csv -o out.bncsv
# Or using bash piping
cat myCsv.csv | bncsv -p -i csv > out.bncsv

```
To decode a BNCSV file use : 
```bash
bncsv out.bncsv -i bncsv > myCsvCopy.csv
# Or directly name the output
bncsv out.bncsv -i bncsv -o myCsvCopy.csv
# Or using bash piping
cat out.bncsv | bncsv -p -i bncsv > myCsvCopy.csv
```

Using glob patterns is possible : 
```bash
bncsv **/*.csv -i csv -o ./outFolder/
bncsv **/*.bncsv -i bncsv -o ./outFolder/

```
When using glob patterns that are absolutes, you need to provide the abs_pathbase argument with `--abs-pathbase` which will help to resolve all the paths in the output dir.

## Technical details
- Static O(1) lookup table to encode UTF-8 chars directly to bits
- Huffman binary tree searching to decode < O(n*log(n)) (with n=14)
- Bit representation is 8bits unsigned integer (0_u8 and 1_u8)
- Number of bits divisible by 8 is reached with a special end of compression char concatenated with zeros bits.
- Mainly iterators and buffering techniques
- Mainly std lib used
- PyO3 Python bindings
