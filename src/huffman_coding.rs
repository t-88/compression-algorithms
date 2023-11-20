use std::{collections::HashMap, char};


#[derive(Debug,Clone)]
pub struct  Node {
    left : Option<Box<FreqNode>>,
    right: Option<Box<FreqNode>>,
}
#[derive(Debug,Clone)]
pub enum NodeType {
    Connector(Node),
    Leaf(char)
}
#[derive(Debug,Clone)]
pub struct  FreqNode {
    val : NodeType,
    freq : u128,
}       



#[derive(Debug)]
pub struct PriorityQueue {
    pub queue : Vec<FreqNode>,
}
impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            queue : vec![],
        }
    }

    fn push(&mut self,data : FreqNode) {
        let mut inserted = false;

        for i in 0..self.queue.len()  {
                if data.freq > self.queue[i as usize].freq {
                    self.queue.insert(i as usize, data.clone());
                    inserted = true;
                    break;
                }
        }
        if !inserted {
            self.queue.push(data.clone());
        }
    }

    fn pop(&mut self) -> FreqNode {
        self.queue.pop().unwrap()
    }
}


#[derive(Debug)]
pub struct HuffmanCoder {
    pub message : String,
    pub encoding_table : HashMap<char,String>,
    char_freq : HashMap<char,u128>,
    node_queue : PriorityQueue, 

}


impl HuffmanCoder {
    pub fn new() -> Self {
        HuffmanCoder { 
            message: "".to_string(),
            char_freq : HashMap::new(),
            encoding_table : HashMap::new(),
            node_queue : PriorityQueue::new(),
        }
    }

    fn fill_encoding_table(&mut self,node : FreqNode,acc : String) {
        match node.val {
            NodeType::Leaf(chr) => {
                self.encoding_table.insert(chr, acc.clone());
            }
            NodeType::Connector(node) => {
                match node.left {
                    Some(freq_node) => self.fill_encoding_table(*freq_node, acc.clone() + "0"),
                    None => (),
                }
                match node.right {
                    Some(freq_node) => self.fill_encoding_table(*freq_node, acc.clone() + "1"),
                    None => (),
                }

            }
        }
    }   

    pub fn display_table(self) {
        println!("Table:");
        for key in self.encoding_table.keys() {
            println!("\t{:?} : {:?}",key,self.encoding_table[key]);
        }
    }


    pub fn encode(&mut self,msg : &String) -> String {
        self.message = msg.to_string();
        let mut out_str = String::from("");

        for chr in self.message.chars()  {
            let freq = match self.char_freq.get(&chr) {
                None => 0,
                Some(x) => *x
            };
            self.char_freq.insert(chr, freq + 1);
        }

        for key in self.char_freq.keys()  {
            self.node_queue.push(
                FreqNode {
                val: NodeType::Leaf(*key), 
                freq: self.char_freq[key]
            });
        }
        if self.node_queue.queue.len() == 0 { 
            return out_str; 
        }


        // sort the node queue
        if self.node_queue.queue.len() == 1 {
            let node_1 = self.node_queue.pop(); 
            self.node_queue.push(
                FreqNode { 
                    freq : node_1.freq,
                    val: NodeType::Connector(
                        Node {
                            left :  Some(Box::new(node_1)),
                            right : None,
                        }
                    )
                }                
            ); 
        } else {
            while self.node_queue.queue.len() > 1 {
                let node_1 = self.node_queue.pop(); 
                let node_2 = self.node_queue.pop();
    
                let node_3 = FreqNode { 
                    freq : node_1.freq + node_2.freq,
                    val: NodeType::Connector(
                        Node {
                            left :  Some(Box::new(node_1)),
                            right : Some(Box::new(node_2)),
                        }
                    )
                };
                self.node_queue.push(node_3); 
            }
        }
        self.fill_encoding_table(self.node_queue.queue[0].clone(),"".to_string());


        // encode msg
        for chr in self.message.chars()  {
            out_str += &self.encoding_table.get(&chr).unwrap();
        }
        out_str
    }


    fn decode_char(&self,msg : &String, idx : &mut usize,node : FreqNode) -> Option<char> {
        match node.val {
            NodeType::Leaf(l) => Some(l),
            NodeType::Connector(connector) => {
                match msg.chars().nth(*idx) {
                    Some(c) => match c {
                        '0' => {
                            match connector.left {
                                Some(n) =>  { 
                                    *idx += 1;
                                    self.decode_char(msg, idx, *n) 
                                },
                                _ => unreachable!("binary sequance cant be decoded!"),
                            }
                        }
                        '1' => {
                            match connector.right {
                                Some(n) =>  { 
                                    *idx += 1;
                                    self.decode_char(msg, idx, *n) 
                                },
                                _ => unreachable!("binary sequance cant be decoded!"),
                            }
                        }
                        _ => unreachable!("expected 0 or 1 got ''{}",c),
                    },
                    _ => unreachable!("encoded string out of range, maybe some data is lost?")
                }
            },
        }

        
    }
    
    pub fn decode(&self,msg : String) -> String {
        let mut out_str = String::from("");
        
        let mut idx = 0;
        while idx < msg.len()  {
            out_str.push(self.decode_char(&msg, &mut idx, self.node_queue.queue[0].clone()).unwrap());
        }
        
        out_str

    }


}


