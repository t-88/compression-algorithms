use std::collections::HashMap;


pub fn generate_freq_table(msg : &String) -> HashMap<char,usize> {
    let mut freq_table = HashMap::new();
    for chr in msg.chars()  {
        let freq = match freq_table.get(&chr) {
            None => 0,
            Some(x) => *x
        };
        freq_table.insert(chr, freq + 1);
    }   
    freq_table
}