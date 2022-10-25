
mod huffman;

fn example() {
    let test_string = "1223334444";
    let tree = huffman::huffman_tree(test_string);
    println!("{:?}", tree);
    let table = huffman::huffman_table(tree, None, None);
    println!("{:?}", table);
    let code: String = String::from_iter(test_string.chars().map(|char| table.get(&char).unwrap().to_owned()));
    println!("{}", code);
}

fn main() {
    example();
}
