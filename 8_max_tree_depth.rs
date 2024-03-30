// Given a binary tree, implement a function that returns the maximum depth of the tree.

// Implemented BST from https://gist.github.com/mikeyhew/21091ecff9c46677fcb0670fb5a7f413

use std::cmp::Ordering::*;

pub struct Node<K: Ord> {
    key: K,
    left: BST<K>,
    right: BST<K>,
}

pub struct BST<K: Ord>(Option<Box<Node<K>>>);

impl<K: Ord + std::fmt::Display> BST<K> {

    pub fn new() -> BST<K> {
        BST(None)
    }

    pub fn insert(&mut self, key: K) -> bool {
        unsafe {
            let mut current_node: *mut BST<K> = self;

            while let Some(ref mut node) = (*current_node).0 {
                match key.cmp(&node.key) {
                    Less => current_node = &mut node.left,
                    Greater => current_node = &mut node.right,
                    Equal => return false
                }
            }

            (*current_node).0 = Some(Box::new(Node {
                key,
                left: BST(None),
                right: BST(None),
            }));

            true
        }
    }

    pub fn remove(&mut self, key: K) -> bool {
        let mut tree: *mut BST<K> = self;

        unsafe {
            while let Some(ref mut node) = (*tree).0 {
                match key.cmp(&node.key) {
                    Less => tree = &mut node.left,
                    Greater => tree = &mut node.right,
                    Equal => {
                        (*tree).0 = None;
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn contains(&self, key: K) -> bool {
        let mut tree = self;

        while let Some(ref node) = tree.0 {
            match key.cmp(&node.key) {
                Less => tree = &node.left,
                Greater => tree = &node.right,
                Equal => return true,
            }
        }

        false
    }

    pub fn max_depth(&self) -> i32 {
        match &self.0 {
            Some(node) => 1 + std::cmp::max(node.left.max_depth(), node.right.max_depth()),
            None => 0,
        }
    }

    pub fn print_pretty(&self, space: usize) {
        match &self.0 {
            Some(current_node) => {
                current_node.right.print_pretty(space + 4);

                for _ in 0..space {
                    print!(" ");
                }

                println!("{}", current_node.key);

                current_node.left.print_pretty(space + 4);
            }
            None => (),
        }
    }
}

fn main() {
    println!("Would you like to use a dummy array? \n 1 => Dummy BST \n 2 => Custom BST");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let node_values: Vec<i32> = if user_selection.trim() == "1" {
        vec![7,5,9,3,20]
    } else if user_selection.trim() == "2" {
        println!("Enter the numbers in the array separated by spaces: ");
        let mut custom_array_input = String::new();
        std::io::stdin().read_line(&mut custom_array_input).expect("Failed to read line");
        custom_array_input
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect()
    } else {
        println!("Invalid input");
        return;
    };

    let mut tree = BST::new();
    for value in node_values {
        tree.insert(value);
    }

    println!("The BST is:");
    tree.print_pretty(0);

    let max_depth = tree.max_depth();
    println!("The maximum depth of the tree is: {}", max_depth);
}
