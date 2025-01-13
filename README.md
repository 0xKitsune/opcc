# OP Compressor Check
Minimal cli tool to evaluate the calldata compression rate on the OP Stack.


## Installation


```bash
git clone https://github.com/0xKitsune/opcc
cd opcc
cargo install --path .
```

## Usage
```bash
Usage: opcc [OPTIONS] --level <LEVEL>

Options:
  -l, --level <LEVEL>  Brotli compression level (9, 10, or 11)
  -d, --data <DATA>    
  -f, --file <FILE>    
  -h, --help           Print help
  -V, --version        Print version
```
