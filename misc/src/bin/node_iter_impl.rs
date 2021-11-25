//! Custom Iter Test
//!
//! https://aloso.github.io/2021/03/09/creating-an-iterator

use std::collections::VecDeque;
use std::iter::FusedIterator;
use std::mem;

fn main() {
    unimplemented!();
}

#[derive(Debug, PartialEq)]
enum Node<Item> {
    Leaf(Item),
    Children(Vec<Node<Item>>),
}

impl<It> Node<It> {
    fn traverse(&self, f: impl Fn(&It)) {
        match self {
            Node::Leaf(item) => {
                f(item);
            }
            Node::Children(children) => {
                for node in children {
                    node.traverse(&f);
                }
            }
        }
    }

    fn iter(&self) -> NodeIter<'_, It> {
        NodeIter {
            children: std::slice::from_ref(self),
            parent: None,
        }
    }

    fn iter_mut(&mut self) -> NodeIterMut<'_, It> {
        NodeIterMut {
            children: std::slice::from_mut(self),
            parent: None,
        }
    }
}

struct NodeIter<'a, It> {
    children: &'a [Node<It>],
    parent: Option<Box<NodeIter<'a, It>>>,
}

impl<It> Default for NodeIter<'_, It> {
    fn default() -> Self {
        NodeIter {
            children: &[],
            parent: None,
        }
    }
}

impl<'a, It> Iterator for NodeIter<'a, It> {
    type Item = &'a It;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(Node::Leaf(item)) => {
                self.children = &self.children[1..];
                Some(item)
            }
            Some(Node::Children(children)) => {
                self.children = &self.children[1..];

                // start iterating the child trees
                *self = NodeIter {
                    children: children.as_slice(),
                    parent: Some(Box::new(mem::take(self))),
                };
                self.next()
            }
        }
    }
}

impl<It> FusedIterator for NodeIter<'_, It> {}

impl<'a, It> IntoIterator for &'a Node<It> {
    type Item = &'a It;

    type IntoIter = NodeIter<'a, It>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct NodeIntoIter<It> {
    children: VecDeque<Node<It>>,
    parent: Option<Box<NodeIntoIter<It>>>,
}

impl<It> Default for NodeIntoIter<It> {
    fn default() -> Self {
        NodeIntoIter {
            children: Default::default(),
            parent: None,
        }
    }
}

impl<It> Iterator for NodeIntoIter<It> {
    type Item = It;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.pop_front() {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(Node::Leaf(item)) => Some(item),
            Some(Node::Children(children)) => {
                // start iterating the child trees
                *self = NodeIntoIter {
                    children: children.into(),
                    parent: Some(Box::new(mem::take(self))),
                };
                self.next()
            }
        }
    }
}

impl<It> FusedIterator for NodeIntoIter<It> {}

impl<It> IntoIterator for Node<It> {
    type Item = It;

    type IntoIter = NodeIntoIter<It>;

    fn into_iter(self) -> Self::IntoIter {
        let mut children = VecDeque::with_capacity(1);
        children.push_back(self);

        NodeIntoIter {
            children,
            parent: None,
        }
    }
}

struct NodeIterMut<'a, It> {
    children: &'a mut [Node<It>],
    parent: Option<Box<NodeIterMut<'a, It>>>,
}

impl<It> Default for NodeIterMut<'_, It> {
    fn default() -> Self {
        NodeIterMut {
            children: &mut [],
            parent: None,
        }
    }
}

impl<'a, It> Iterator for NodeIterMut<'a, It> {
    type Item = &'a mut It;

    fn next(&mut self) -> Option<Self::Item> {
        let children = mem::take(&mut self.children);
        match children.split_first_mut() {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some((first, rest)) => {
                self.children = rest;
                match first {
                    Node::Leaf(item) => Some(item),
                    Node::Children(children) => {
                        *self = NodeIterMut {
                            children: children.as_mut_slice(),
                            parent: Some(Box::new(mem::take(self))),
                        };
                        self.next()
                    }
                }
            }
        }
    }
}

impl<'a, It> IntoIterator for &'a mut Node<It> {
    type Item = &'a mut It;

    type IntoIter = NodeIterMut<'a, It>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[test]
pub fn test_owned_iterator() {
    let tree = Node::Children(vec![
        Node::Leaf(5),
        Node::Leaf(4),
        Node::Children(vec![Node::Leaf(3), Node::Leaf(2), Node::Children(vec![])]),
        Node::Children(vec![Node::Children(vec![
            Node::Children(vec![Node::Leaf(1)]),
            Node::Leaf(0),
        ])]),
    ]);

    let nums: Vec<i32> = tree.into_iter().collect();
    assert_eq!(nums, vec![5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_owned_for_loop() {
    let tree = Node::Leaf(42);

    for node in tree {
        let _: i32 = node;
    }
}

#[test]
fn test_borrowing_iterator() {
    let tree = Node::Children(vec![
        Node::Leaf(5),
        Node::Leaf(4),
        Node::Children(vec![Node::Leaf(3), Node::Leaf(2), Node::Children(vec![])]),
        Node::Children(vec![Node::Children(vec![
            Node::Children(vec![Node::Leaf(1)]),
            Node::Leaf(0),
        ])]),
    ]);

    let nums: Vec<i32> = tree.iter().copied().collect();
    assert_eq!(nums, vec![5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_borrowing_for_loop() {
    let tree = Node::Leaf(42);

    for &node in &tree {
        let _: i32 = node;
    }
}

#[test]
fn test_mut_iterator() {
    let mut tree = Node::Children(vec![
        Node::Leaf(5),
        Node::Leaf(4),
        Node::Children(vec![Node::Leaf(3), Node::Leaf(2), Node::Children(vec![])]),
        Node::Children(vec![Node::Children(vec![
            Node::Children(vec![Node::Leaf(1)]),
            Node::Leaf(0),
        ])]),
    ]);

    for item in &mut tree {
        *item += 1;
    }

    assert_eq!(
        tree,
        Node::Children(vec![
            Node::Leaf(6),
            Node::Leaf(5),
            Node::Children(vec![Node::Leaf(4), Node::Leaf(3), Node::Children(vec![])]),
            Node::Children(vec![Node::Children(vec![
                Node::Children(vec![Node::Leaf(2)]),
                Node::Leaf(1),
            ])]),
        ])
    );
}
