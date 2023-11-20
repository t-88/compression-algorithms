# Compression
learning about compression from this [book](./compression.pdf)

# Done List
- [X] Huffman Codes 

**CHECK EXAMPLES FOR HOW TO USE** 
## Huffman coding:
-   pretty simple and *straightforward* algorithm :P
```rust
    let mut coder = huffman_coding::HuffmanCoder::new(); // init struct
    let encoded_str = coder.encode(&input_str);          // then just encode
    let decoded_str = coder.decode(encoded_str.clone()); // decode by giving it the encoded str back
    coder.display_table();                               // display symbol table1
```

