use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub struct HuffmanCoding {
    pub table: HashMap<char, String>,
    pub encoded: String,
}

#[derive(Debug, Eq, PartialEq)]
enum NodeType {
    Leaf(char),
    Internal {
        left_child: Box<Node>,
        right_child: Box<Node>,
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    freq: i32,
    data: NodeType,
}

/// Explicitly implement Ord for Node so BinaryHeap<Node> becomes a min-heap
/// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl HuffmanCoding {
    pub fn encode(s: &str) -> HuffmanCoding {
        let freq_map = HuffmanCoding::build_freq_map(s);
        let tree_root = HuffmanCoding::build_huffman_tree(&freq_map);

        HuffmanCoding {
            table: HashMap::new(),
            encoded: String::new(),
        }
    }

    fn build_freq_map(s: &str) -> HashMap<char, i32> {
        let mut freq_map = HashMap::new();
        for c in s.chars() {
            let mut freq = match freq_map.get(&c) {
                Some(freq) => *freq,
                None => 0
            };
            freq += 1;
            freq_map.insert(c, freq);
        }
        freq_map
    }

    fn build_huffman_tree(freq_map: &HashMap<char, i32>) -> Node {
        let mut min_heap = BinaryHeap::new();
        for (c, freq) in freq_map.iter() {
            min_heap.push(Node {
                freq: *freq,
                data: NodeType::Leaf(*c),
            });
        }

        while min_heap.len() > 1 {
            let node1 = min_heap.pop().unwrap();
            let node2 = min_heap.pop().unwrap();
            min_heap.push(Node {
                freq: node1.freq + node2.freq,
                data: NodeType::Internal {
                    left_child: Box::new(node1),
                    right_child: Box::new(node2),
                }
            });
        }

        min_heap.pop().unwrap()
    }
}

fn main() {
    let s = String::from("encode this huffman string");
    let huffman = HuffmanCoding::encode(&s);
    println!("{}", huffman.encoded);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huffman_tree() {
        let mut input = String::new();
        for _ in 0..5 {
            input.push('a');
        }
        for _ in 0..9 {
            input.push('b');
        }
        for _ in 0..12 {
            input.push('c');
        }
        for _ in 0..13 {
            input.push('d');
        }
        for _ in 0..16 {
            input.push('e');
        }
        for _ in 0..45 {
            input.push('f');
        }

        let freq_map = HuffmanCoding::build_freq_map(&input);
        let huffman_tree = HuffmanCoding::build_huffman_tree(&freq_map);

        assert_eq!(huffman_tree, Node {
            freq: 100,
            data: NodeType::Internal {
                left_child: Box::new(Node {
                    freq: 45,
                    data: NodeType::Leaf('f')
                }),
                right_child: Box::new(Node {
                    freq: 55,
                    data: NodeType::Internal {
                        left_child: Box::new(Node {
                            freq: 25,
                            data: NodeType::Internal {
                                left_child: Box::new(Node {
                                    freq: 12,
                                    data: NodeType::Leaf('c')
                                }),
                                right_child: Box::new(Node {
                                    freq: 13,
                                    data: NodeType::Leaf('d')
                                }) 
                            }
                        }),
                        right_child: Box::new(Node {
                            freq: 30,
                            data: NodeType::Internal {
                                left_child: Box::new(Node {
                                    freq: 14,
                                    data: NodeType::Internal {
                                        left_child: Box::new(Node {
                                            freq: 5,
                                            data: NodeType::Leaf('a')
                                        }),
                                        right_child: Box::new(Node {
                                            freq: 9,
                                            data: NodeType::Leaf('b')
                                        })
                                    }
                                }),
                                right_child: Box::new(Node {
                                    freq: 16,
                                    data: NodeType::Leaf('e')
                                }) 
                            }
                        })
                    }
                })
            }
        });
    }
}
