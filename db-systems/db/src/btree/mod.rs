mod node;
mod stack;

use node::InsertionResult::*;
use node::Node;
use node::SearchResult::*;
use stack::*;

use std::mem;

pub struct BTreeMap<K, V> {
    root: Node<K, V>,
    length: usize,
    depth: usize,
    b: usize,
}

impl<K, V> BTreeMap<K, V> {
    pub fn new() -> Self {
        // getting these results with the movies table
        // - length: 27278,
        // - depth: 5,
        // - b: 6,
        // Self::with_b(6)

        // changed to 24 to make smaller depth (kinda like b+tree)
        // - length: 27278,
        // - depth: 3,
        // - b: 24,
        Self::with_b(24)
    }
    fn with_b(b: usize) -> Self {
        Self {
            root: Node::make_leaf_root(b),
            length: 0,
            depth: 1,
            b,
        }
    }
}

use core::borrow::Borrow;

impl<K: Ord, V> BTreeMap<K, V> {
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let mut curr = &self.root;
        loop {
            match curr.search(key) {
                Found(i) => return curr.val(i),
                GoDown(i) => {
                    let next = curr.edge(i)?;
                    curr = next;
                    continue;
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        use stack::*;

        let mut stack = PartialSearchStack::new(self);

        loop {
            match stack.next().search(&key) {
                Found(i) => unsafe {
                    let next = stack.into_next();
                    mem::swap(next.unsafe_val_mut(i), &mut value);
                    return Some(value);
                },
                GoDown(i) => {
                    stack = match stack.push(i) {
                        PushResult::Done(new_stack) => {
                            new_stack.insert(key, value);
                            return None;
                        }
                        PushResult::Grew(new_stack) => new_stack,
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut stack = PartialSearchStack::new(self);
        loop {
            match stack.next().search(key) {
                Found(i) => {
                    return Some(stack.seal(i).remove());
                }
                GoDown(i) => {
                    stack = match stack.push(i) {
                        PushResult::Done(_) => return None,
                        PushResult::Grew(new_stack) => new_stack,
                    };
                }
            }
        }
    }
}

use std::fmt;

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for BTreeMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BTreeMap")
         .field("root", &self.root)
         .field("length", &self.length)
         .field("depth", &self.depth)
         .field("b", &self.b)
         .finish()
    }
}
