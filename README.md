# Dummy arrays

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)


## General conception and implementation 📑
This little project aims to implement _an associative data structure_, called Dummy-array.<br/>
It would works around three entities, as follows :
- one sub data structure _for indexing_.
- another _to store values_.
- and a counter _to keep track of the next index to write onto_.

The idea is to _access only the first sub data structure_, containing **pointers to the other one** where the values are stored.
Both sub data structures have the same lenght, and **the indexes of the first one** define the value the relative pointer
is **expected to point to**. In other words, the **lenght-1 of the dummy-array corresponds to the maximum value** it can store (from
0 to lenght-1).

This data structure must implements three methods :
- **`exists(int value)`** -> return True if the given value is already stored; else False.
- **`add(int value)`** -> if the given values is not already stored, insert it and return True; else return False.
- **`remove(int value)`** -> if the given value is already stored, delete it and return True; else return False.

For example, if the pointer at index 3 doese not point to a slot containing 3, it means the value isn't stored in the dummy-array.
To mark a slot as writtable in the storing sub data structure, it is field with the lenght of the dummy-array. So if we want to 
add a value we therefore need to search for the first slot marked thus. And remove a value is as simple as write the lenght in the 
concerned slot.
Every operation, execpt the initialization, should be process from the indexing sub data structure, by dereferencing.

## Download and setup 🚂 
⚠️ The following operations have to be done within a linux environnement. 

If you'd like to try it out for yourself, we've set up a _ready-to-use_ **Qemu** virtual machine for you. You can download it here:<br/>
*link coming soon*

To import a
*instructions sur l'import d'une vm qemu*

*instructions sur les .sh à lancer*

## Benchmark 📊
We so choose to implement a Dummy-array in Go and Rust, then compared the results between the two languages during the benchmark.
