extern crate smallbitvec;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use smallbitvec::SmallBitVec;

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

pub struct HuffmanCode {
    pub code_table: HashMap<char, String>,
    pub compressed: SmallBitVec,
}

pub fn huffman_code(s: &str) -> HuffmanCode {
    let freq_map = build_freq_map(s);
    let tree = build_huffman_tree(&freq_map);
    let code_table = build_code_table(tree);
    let compressed = compress(s, &code_table);

    HuffmanCode {
        code_table: code_table,
        compressed: compressed,
    }
}

pub fn huffman_decode(huffman_code: &HuffmanCode) -> String {
    String::new()
}

fn build_freq_map(s: &str) -> HashMap<char, i32> {
    let mut freq_map = HashMap::new();
    for c in s.chars() {
        let freq = freq_map.entry(c).or_insert(0);
        *freq += 1;
    }
    freq_map
}

fn build_huffman_tree(freq_map: &HashMap<char, i32>) -> Node {
    let mut min_heap = BinaryHeap::new();

    // Populate the min-heap with all unique characters.
    for (c, freq) in freq_map.iter() {
        min_heap.push(Node {
            freq: *freq,
            data: NodeType::Leaf(*c),
        });
    }

    while min_heap.len() > 1 {
        // Pop two minimum frequency nodes off the heap.
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

    // Return the root node.
    min_heap.pop().unwrap()
}

fn build_code_table(root: Node) -> HashMap<char, String> {
    let mut table = HashMap::new();
    let mut node_stack = Vec::new();
    node_stack.push((root, String::new()));

    while !node_stack.is_empty() {
        let (node, code) = node_stack.pop().unwrap();
        match node.data {
            NodeType::Internal { left_child, right_child } => {
                let left_child = *left_child;
                let right_child = *right_child;
                node_stack.push((left_child, code.clone() + "0"));
                node_stack.push((right_child, code + "1"));
            },
            NodeType::Leaf(sym) => {
                // We've reached the end of a branch, add this code point to the table.
                table.insert(sym, code);
            },
        }
    }

    table
}

fn compress(s: &str, table: &HashMap<char, String>) -> SmallBitVec {
    s.chars()
        .map(|c| table.get(&c).unwrap())
        .flat_map(|s| s.chars())
        .map(|c| if c == '1' { true } else { false })
        .collect()
}

fn main() {
    let s = String::from("encode this huffman string");
    let huffman = huffman_code(&s);
    println!("{:?}", huffman.compressed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huffman_tree() {
        let input = build_input();
        let freq_map = build_freq_map(&input);
        let huffman_tree = build_huffman_tree(&freq_map);

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

    #[test]
    fn code_table() {
        let input = build_input();
        let freq_map = build_freq_map(&input);
        let huffman_tree = build_huffman_tree(&freq_map);
        let code_table = build_code_table(huffman_tree);
        let mut correct_code_table = HashMap::new();
        correct_code_table.insert('f', String::from("0"));
        correct_code_table.insert('c', String::from("100"));
        correct_code_table.insert('d', String::from("101"));
        correct_code_table.insert('a', String::from("1100"));
        correct_code_table.insert('b', String::from("1101"));
        correct_code_table.insert('e', String::from("111"));

        assert_eq!(code_table, correct_code_table);
    }

    #[test]
    fn encode_decode() {
        let input = "this should work";
        let huffman = huffman_code(&input);
        assert_eq!(input, huffman_decode(&huffman)); 
    }

    fn build_input() -> String {
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
        input
    }
}
