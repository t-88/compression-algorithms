use std::{collections::HashMap, char};
use crate::{shared};

pub struct ArithmeticCoder {
    msg : String,
    prob_table : HashMap<char,(f32,f32)>,
}

impl ArithmeticCoder {
    pub fn new() -> Self {
        ArithmeticCoder {
           msg : String::new(),
           prob_table : HashMap::new(),
        }
    }

    pub fn display_table(self) {
        println!("Table:");
        for (k,v) in self.prob_table.iter() {
            println!("\t{} : [{},{})",k,v.0,v.1);
        }
    }

    pub fn encode(&mut self,msg : &String) -> f32 {

        self.msg = msg.clone();
        //NOTE: my end of msg char is '@'
        self.msg.push('@');

        let freq_table = shared::generate_freq_table(&self.msg);


        
        let mut com_min  = 0.0;
        let mut com_max  = 0.0;
        let mut prv_max = 0.0;
        for k in freq_table.keys() {
            let prob = (freq_table[k] as f32) / self.msg.len() as f32;
            com_min += prv_max;
            prv_max = prob;
            com_max += prob;
            self.prob_table.insert(
                *k,
                (com_min,com_max)
            );
        }
        

        let mut low = 0.0;
        let mut high = 1.0;

        for chr in self.msg.chars() {
            let r_min = self.prob_table[&chr].0;
            let r_max = self.prob_table[&chr].1;
            let w = high - low;             

            let tmp_min = low;
            low = tmp_min + r_min * w;
            high = tmp_min + r_max * w;
        }
        (high + low) / 2.0
    }


    pub fn decoder_letter_from_range(&self,msg : f32) -> Option<(char,f32,f32)> {
        for (k , v) in self.prob_table.iter() {
            if v.0 <= msg && msg <= v.1 {
                return Some((*k,v.0,v.1))
            }
        }
        None
    }
    pub fn decode(&self ,msg : f32) -> String {
        let mut out_str = String::from("");
        let mut message = msg;
        
        loop {
            let info = self.decoder_letter_from_range(message).unwrap(); 
            message = (message - info.1) / (info.2 - info.1);
            if info.0 == '@' { break; }
            out_str.push(info.0);
        }
        out_str
    }

}