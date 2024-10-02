use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    child: HashMap<char, TrieNode>,
    is_end_node: bool
}

#[derive(Default, Debug)]
pub struct Trie {
    node: TrieNode
}

impl Trie {
    pub fn new() -> Trie {
        Trie { node: TrieNode { child: HashMap::new(), is_end_node: true } }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.node;

        for cha in word.chars() {
            node = node.child.entry(cha).or_default();
        }
        node.is_end_node = true
    }

    pub fn contain(&self, word: &str) -> bool {
        let mut current_node = &self.node;

        for cha in word.chars() {
            match current_node.child.get(&cha) {
                Some(node) =>  current_node = &node,
                None => return  false,
            }
        }

        current_node.is_end_node
    }
}

#[cfg(test)]
mod test {
    use super::Trie;

    #[test]
    fn test() {
        let mut trie = Trie::new();

        trie.insert("Hello");
        trie.insert("Hi");
        trie.insert("hey");
        trie.insert("redAndBlue");


        assert!(trie.contain("Hello") == true);
        assert!(trie.contain("hey") == true);
        assert!(trie.contain("hewwwo") == false);

    }
}

