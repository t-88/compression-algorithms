use crate::huffman_coding;
use crate::arithmetic_coding;
use crate::shared;

pub fn huffman_coding_example() {
    let input_str = shared::get_input();

    let mut coder = huffman_coding::HuffmanCoder::new();
    let encoded_str = coder.encode(&input_str);
    let decoded_str = coder.decode(encoded_str.clone());

    
    coder.display_table();
    println!("\ninput  : {:?}\nencoded: {:?}\ndecoded: {:?}\n",input_str,encoded_str,decoded_str);
}

pub fn arithmetic_coding_example() {
    let input_str = shared::get_input();
    let mut coder = arithmetic_coding::ArithmeticCoder::new();
    let encoded_number = coder.encode(&input_str);
    let decoded_str =  coder.decode(encoded_number);

    coder.display_table();
    println!("input: {}\nencoded float: {}\ndecoded: {}",input_str,encoded_number,decoded_str);

}