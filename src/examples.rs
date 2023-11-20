use std;

use crate::huffman_coding;
pub fn huffman_example() {
    let mut input_str = "".to_string();
    println!("input str to decode:");
    std::io::stdin().read_line(&mut input_str).unwrap();
    println!();

    let mut coder = huffman_coding::HuffmanCoder::new();
    let encoded_str = coder.encode(&input_str);
    let decoded_str = coder.decode(encoded_str.clone());

    
    coder.display_table();
    println!("\ninput  : {:?}\nencoded: {:?}\ndecoded: {:?}\n",input_str,encoded_str,decoded_str);
}