extern crate crypto;

use crypto::sha2::Sha256;
use crypto::digest::Digest;
//use std::slice::raw;
//use std::mem;
//use std::fmt;

trait Hash {
    fn hash<'a>(&'a self) -> &'a [u8];
}

impl Hash for String {
    fn hash<'a>(&'a self) -> &'a [u8] {
        self.as_slice().as_bytes()
    }
}

impl<'a> Hash for &'a str {
    fn hash(&'a self) -> &'a [u8] {
        self.as_bytes()
    }
}

fn hash<T: Hash>(input: T) -> String {
    let mut hash = Sha256::new();
    hash.input(input.hash());
    hash.result_str()
}

//#[deriving(Show)]
pub struct MerkleTree<T> {
    hash: String,
    root: Node<T>
}

impl<T> MerkleTree<T>
    where T: Hash {
    pub fn new(root: Node<T>) -> MerkleTree<T> {
        MerkleTree {
            hash: String::new(),
            root: root
        }
    }

    pub fn hash(&mut self) {
        self.root.hash();
        if self.root.children.len() == 0 {
            self.hash = self.root.value_hash.clone();
        } else {
            self.hash = self.root.combined_hash.clone();
        }
    }

    pub fn root<'a>(&'a self) -> &'a str {
        self.hash.as_slice()
    }
}

//#[deriving(debug)]
pub struct Node<T> {
    value_hash: String,
    combined_hash: String,
    children: Vec<Node<T>>
}

impl<T> Node<T>
    where T: Hash {
    pub fn new(value: T) -> Node<T> {
        Node {
            value_hash: hash(value),
            combined_hash: String::new(),
            children: Vec::new()
        }
    }

    pub fn hash(&mut self) {
        let mut combined = String::new();
        for child in self.children.iter_mut() {
            child.hash();

            if child.children.len() == 0 {
                combined = combined + child.value_hash;
            } else {
                combined = combined + child.combined_hash;
            }
        }

        if self.children.len() > 0 {
            self.combined_hash = hash(self.value_hash + combined);
        }
    }

    pub fn push_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }
}

#[cfg(test)]
mod test {
    use super::{MerkleTree, Node};

    #[test]
    fn new_root_node() {
        let root = Node::new("");
        let mut merkle = MerkleTree::new(root);
        merkle.hash();
        assert_eq!(merkle.root(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    }

    #[test]
    fn one_child() {
        let mut root = Node::new("");
        root.push_child(Node::new(""));
       // let mut merkle = MerkleTree::new(root);
       // merkle.hash();
       // assert_eq!(merkle.root(), "3b7546ed79e3e5a7907381b093c5a182cbf364c5dd0443dfa956c8cca271cc33");
       // assert_eq!(merkle.root.children[0].value_hash.as_slice(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }
}
