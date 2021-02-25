<img src="resources/logo.svg" width=80px)/>

# rudoko

Sudoku solver in Rust. 

# CLI Reference
```
rudoku 0.1
D. Kleiven
Solves sudoku's using a brute force search. The sudoku is given as via a text file where  each line corresponds to a
row. Unknown values are represented with the x character.
Example:
xxxxx7xxx
x65xxx89x
4xx5x1xx7
89x1x5x2x
xx26x43xx
x4x9x3x1x
3xx7x8xx4
x57xxx98x
xxxx3xxxx

USAGE:
    rudoko [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>    Text file containing the sudoku
    -n, --nmax <nmax>      Maximum number of steps used in the search. If -1 no limit is imposed [default: -1]
```