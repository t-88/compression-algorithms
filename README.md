# Compression
reading about compression for some reason.

## Dev Notes:
this is a implemntaion for the algorthims, that means i stop with the algorithm output but a real implementation accounts for the probability table or smth like that and the final binary output.  

# Resources
check out [resources](./resources.md)

# Done List
- [X] Huffman Codes 
- [X] Arithmetic Coding 

**CHECK EXAMPLES FOR HOW TO USE** 
## Huffman coding:
-   pretty simple and *straightforward* algorithm :P
```rust
    let mut coder = huffman_coding::HuffmanCoder::new(); // init struct
    let encoded_str = coder.encode(&input_str);          // then just encode
    let decoded_str = coder.decode(encoded_str.clone()); // decode by giving it the encoded str back
    coder.display_table();                               // display symbol table
```
## Arithmetic Coding:
-   using conditional probabily and scaling floats to decode a message is on a  whole new lvl, Nice :) 
-   u get a float as the encoded message xd, its pretty funny 
```rust
    let mut coder = arithmetic_coding::ArithmeticCoder::new();  // init struct
    let encoded_number = coder.encode(&input_str);              // ask for encoding u get a float
    let decoded_str =  coder.decode(encoded_number);            // decode that float
    coder.display_table();                                      // symbol table
```

