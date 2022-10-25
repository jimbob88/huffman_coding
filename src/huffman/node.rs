
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Node {
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
    pub freq: i32,
    pub char: Option<char>
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compares the wrong way around, so that binary heap is reverse order.
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    use super::*;

    #[test]
    fn test_node_comparison() {
        let smaller_node = Node {left: None, right: None, freq: 1, char: None};
        let bigger_node =  Node {left: None, right: None, freq: 10, char: None};
        
        assert_eq!(smaller_node.cmp(&bigger_node), Ordering::Greater);
        assert_eq!(bigger_node.cmp(&smaller_node), Ordering::Less);
    }

    #[test]
    fn test_order_in_binary_heap() {
        let mut nodes = BinaryHeap::new();
        
        for (char, count) in [('a', 1), ('b', 2)] {
            nodes.push(Node {left: None, right: None, freq: count, char: Some(char)});
        }

        assert_eq!(nodes.pop().unwrap(), Node {left: None, right: None, freq: 1, char: Some('a')});
        assert_eq!(nodes.pop().unwrap(), Node {left: None, right: None, freq: 2, char: Some('b')});
    }

    #[test]
    fn test_order_in_binary_heap_with_different_order() {
        let mut nodes = BinaryHeap::new();
        
        for (char, count) in [('b', 2), ('a', 1)] {
            nodes.push(Node {left: None, right: None, freq: count, char: Some(char)});
        }

        assert_eq!(nodes.pop().unwrap(), Node {left: None, right: None, freq: 1, char: Some('a')});
        assert_eq!(nodes.pop().unwrap(), Node {left: None, right: None, freq: 2, char: Some('b')});
    }

    #[test]
    fn test_order_in_binary_heap_after_box() {
        let mut nodes = BinaryHeap::new();
        
        for (char, count) in [('b', 2), ('a', 1), ('c', 3)] {
            nodes.push(Node {left: None, right: None, freq: count, char: Some(char)});
        }

        let left = Box::new(nodes.pop().unwrap());
        let right = Box::new(nodes.pop().unwrap());
        let l_freq = left.freq;
        let r_freq = right.freq;
        nodes.push(Node {left: Some(left), right: Some(right), freq: l_freq + r_freq, char: None});

        assert_eq!(nodes.pop().unwrap(), Node {left: None, right: None, freq: 3, char: Some('c')});
        assert_eq!(nodes.pop().unwrap(), 
                    Node { left: Some(Box::new(Node { left: None, right: None, freq: 1, char: Some('a') })), 
                    right: Some(Box::new(Node { left: None, right: None, freq: 2, char: Some('b') })), freq: 3, char: None }
        );
    }

}
