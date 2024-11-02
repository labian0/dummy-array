# Dummy arrays

## General conception and implementation
This little project aims to implement _an associative data structure_, called Dummy-array.<br/>
It would works around three entities, as follows :
- one sub data structure _for indexing_.
- another _to store values_.
- and a counter _to keep track of the next index to write onto_.

This data structure must implements three methods :
- **`exists(int value)`** -> return True if the given value is already stored; else False.
- **`add(int value)`** -> if the given values is not already stored, insert it and return True; else return False.
- **`remove(int value)`** -> if the given value is already stored, delete it and return True; else return False 

## Benchmark
We so choose to implement a Dummy-array in Go and Rust, then compared the results between the two languages during the benchmark.
