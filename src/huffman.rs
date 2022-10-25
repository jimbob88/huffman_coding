use std::collections::HashMap;
use std::collections::BinaryHeap;

mod node;
mod count;

pub fn huffman_tree(string: &'static str) -> node::Node {
    let counts = count::ordered_count_chars(string);

    let mut nodes = BinaryHeap::new();
    
    for (char, count) in counts {
        nodes.push(node::Node {left: None, right: None, freq: count, char: Some(char)});
    }

    while nodes.len() > 1 {
        let left = Box::new(nodes.pop().unwrap());
        let right = Box::new(nodes.pop().unwrap());
        let l_freq = left.freq;
        let r_freq = right.freq;
        nodes.push(node::Node {left: Some(left), right: Some(right), freq: l_freq + r_freq, char: None})
    }

    nodes.pop().unwrap()
}

pub fn huffman_table(tree: node::Node, left: Option<&str>, right: Option<&str>) -> HashMap<char, String> {

    let lc = left.unwrap_or("0");
    let rc = right.unwrap_or("1");

    let mut stack: Vec<(node::Node, String)> = Vec::new();
    stack.push((tree, "".to_string()));
    let mut table: HashMap<char, String> = HashMap::new();

    while !stack.is_empty() {
        let (node, parent) = stack.pop().unwrap();
        println!("{:?}", node); 
        if node.char.is_some() && node.left.is_none() && node.right.is_none() {
            table.insert(node.char.unwrap(), parent.clone());
        }

        if node.left.is_some() {
            stack.push((*node.left.unwrap().clone(), parent.clone() + lc));
        }
        if node.right.is_some() {
            stack.push((*node.right.unwrap().clone(), parent.clone() + rc));
        }

    }

    table
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_huffman_tree() {
        let string = "1223334444";
        let tree = huffman_tree(string);    
        
        let four = node::Node { left: None, right: None, freq: 4, char: Some('4') };
        let three = node::Node { left: None, right: None, freq: 3, char: Some('3') };
        let one = node::Node { left: None, right: None, freq: 1, char: Some('1') };
        let two  = node::Node { left: None, right: None, freq: 2, char: Some('2') };

        assert_eq!(tree, node::Node { left: Some(Box::new(four)), right: Some(Box::new(node::Node { left: Some(Box::new(three)), right: Some(Box::new(node::Node { left: Some(Box::new(one)), right: Some(Box::new(two)), freq: 3, char: None })), freq: 6, char: None })), freq: 10, char: None })

    }

    #[test]
    fn test_huffman_tree_is_constant() {
        let string1 = "1223334444";
        let string2 = "1224334344";
        assert_eq!(huffman_tree(string1), huffman_tree(string2));
    }

    #[cfg(feature="torture")]
    #[test]
    fn test_huffman_tree_is_constant_torture() {
        for _ in 0..1_000 {
            test_huffman_tree_is_constant();
        }
    }

    #[test]
    fn test_huffman_table() {
        let string = "12";
        let tree = huffman_tree(string);    
        let table = huffman_table(tree, Some("0"), Some("1"));
        assert_eq!(table.get(&'1').unwrap(), "0");
        assert_eq!(table.get(&'2').unwrap(), "1");
    }

}