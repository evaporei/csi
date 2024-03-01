use super::BTreeMap;
use super::*;

type StackItem<K, V> = (*mut Node<K, V>, usize);
type Stack<K, V> = Vec<StackItem<K, V>>;

/// A PartialSearchStack handles the construction of a search stack.
pub struct PartialSearchStack<'a, K: 'a, V: 'a> {
    map: &'a mut BTreeMap<K, V>,
    stack: Stack<K, V>,
    next: *mut Node<K, V>,
}

/// A SearchStack represents a full path to an element of interest. It provides methods
/// for manipulating the element at the top of its stack.
pub struct SearchStack<'a, K: 'a, V: 'a> {
    map: &'a mut BTreeMap<K, V>,
    stack: Stack<K, V>,
    top: StackItem<K, V>,
}

/// The result of asking a PartialSearchStack to push another node onto itself. Either it
/// Grew, in which case it's still Partial, or it found its last node was actually a leaf, in
/// which case it seals itself and yields a complete SearchStack.
pub enum PushResult<'a, K: 'a, V: 'a> {
    Grew(PartialSearchStack<'a, K, V>),
    Done(SearchStack<'a, K, V>),
}

use PushResult::*;

impl<'a, K, V> PartialSearchStack<'a, K, V> {
    /// Creates a new PartialSearchStack from a BTreeMap by initializing the stack with the
    /// root of the tree.
    pub fn new(map: &'a mut BTreeMap<K, V>) -> PartialSearchStack<'a, K, V> {
        let depth = map.depth;

        PartialSearchStack {
            next: &mut map.root as *mut _,
            map,
            stack: Vec::with_capacity(depth),
        }
    }

    /// Pushes the requested child of the stack's current top on top of the stack. If the child
    /// exists, then a new PartialSearchStack is yielded. Otherwise, a full SearchStack is
    /// yielded.
    pub fn push(self, edge: usize) -> PushResult<'a, K, V> {
        let map = self.map;
        let mut stack = self.stack;
        let next_ptr = self.next;
        let next_node = unsafe { &mut *next_ptr };
        let to_insert = (next_ptr, edge);
        match next_node.edge_mut(edge) {
            None => Done(SearchStack {
                map,
                stack,
                top: to_insert,
            }),
            Some(node) => {
                stack.push(to_insert);
                Grew(PartialSearchStack {
                    map,
                    stack,
                    next: node as *mut _,
                })
            }
        }
    }

    /// Converts the stack into a mutable reference to its top.
    pub fn into_next(self) -> &'a mut Node<K, V> {
        unsafe { &mut *self.next }
    }

    /// Gets the top of the stack.
    pub fn next(&self) -> &Node<K, V> {
        unsafe { &*self.next }
    }

    /// Converts the PartialSearchStack into a SearchStack.
    pub fn seal(self, index: usize) -> SearchStack<'a, K, V> {
        SearchStack {
            map: self.map,
            stack: self.stack,
            top: (self.next as *mut _, index),
        }
    }
}

impl<'a, K, V> SearchStack<'a, K, V> {
    /// Inserts the key and value into the top element in the stack, and if that node has to
    /// split recursively inserts the split contents into the next element stack until
    /// splits stop.
    ///
    /// Assumes that the stack represents a search path from the root to a leaf.
    ///
    /// An &mut V is returned to the inserted value, for callers that want a reference to this.
    pub fn insert(self, key: K, val: V) -> &'a mut V {
        unsafe {
            let map = self.map;
            map.length += 1;

            let mut stack = self.stack;
            // Insert the key and value into the leaf at the top of the stack
            let (node, index) = self.top;
            let (mut insertion, inserted_ptr) = { (*node).insert_as_leaf(index, key, val) };

            loop {
                match insertion {
                    Fit => {
                        // The last insertion went off without a hitch, no splits! We can stop
                        // inserting now.
                        return &mut *inserted_ptr;
                    }
                    Split(key, val, right) => match stack.pop() {
                        // The last insertion triggered a split, so get the next element on the
                        // stack to recursively insert the split node into.
                        None => {
                            // The stack was empty; we've split the root, and need to make a
                            // a new one. This is done in-place because we can't move the
                            // root out of a reference to the tree.
                            Node::make_internal_root(&mut map.root, map.b, key, val, right);

                            map.depth += 1;
                            return &mut *inserted_ptr;
                        }
                        Some((node, index)) => {
                            // The stack wasn't empty, do the insertion and recurse
                            insertion = (*node).insert_as_internal(index, key, val, right);
                            continue;
                        }
                    },
                }
            }
        }
    }

    /// Removes the key and value in the top element of the stack, then handles underflows as
    /// described in BTree's pop function.
    pub fn remove(mut self) -> V {
        // Ensure that the search stack goes to a leaf. This is necessary to perform deletion
        // in a BTree. Note that this may put the tree in an inconsistent state (further
        // described in leafify's comments), but this is immediately fixed by the
        // removing the value we want to remove
        self.leafify();

        let map = self.map;
        map.length -= 1;

        let mut stack = self.stack;

        // Remove the key-value pair from the leaf that this search stack points to.
        // Then, note if the leaf is underfull, and promptly forget the leaf and its ptr
        // to avoid ownership issues.
        let (value, mut underflow) = unsafe {
            let (leaf_ptr, index) = self.top;
            let leaf = &mut *leaf_ptr;
            let (_key, value) = leaf.remove_as_leaf(index);
            let underflow = leaf.is_underfull();
            (value, underflow)
        };

        loop {
            match stack.pop() {
                None => {
                    // We've reached the root, so no matter what, we're done. We manually
                    // access the root via the tree itself to avoid creating any dangling
                    // pointers.
                    if map.root.len() == 0 && !map.root.is_leaf() {
                        // We've emptied out the root, so make its only child the new root.
                        // If it's a leaf, we just let it become empty.
                        map.depth -= 1;
                        map.root = map.root.pop_edge().unwrap();
                    }
                    return value;
                }
                Some((parent_ptr, index)) => {
                    if underflow {
                        // Underflow! Handle it!
                        unsafe {
                            let parent = &mut *parent_ptr;
                            parent.handle_underflow(index);
                            underflow = parent.is_underfull();
                        }
                    } else {
                        // All done!
                        return value;
                    }
                }
            }
        }
    }

    /// Subroutine for removal. Takes a search stack for a key that might terminate at an
    /// internal node, and mutates the tree and search stack to *make* it a search stack
    /// for that same key that *does* terminates at a leaf. If the mutation occurs, then this
    /// leaves the tree in an inconsistent state that must be repaired by the caller by
    /// removing the entry in question. Specifically the key-value pair and its successor will
    /// become swapped.
    fn leafify(&mut self) {
        unsafe {
            let (node_ptr, index) = self.top;
            // First, get ptrs to the found key-value pair
            let node = &mut *node_ptr;
            let (key_ptr, val_ptr) = {
                (
                    node.unsafe_key_mut(index) as *mut _,
                    node.unsafe_val_mut(index) as *mut _,
                )
            };

            // Try to go into the right subtree of the found key to find its successor
            match node.edge_mut(index + 1) {
                None => {
                    // We're a proper leaf stack, nothing to do
                }
                Some(mut temp_node) => {
                    //We're not a proper leaf stack, let's get to work.
                    self.stack.push((node_ptr, index + 1));
                    loop {
                        // Walk into the smallest subtree of this node
                        let node = temp_node;
                        let node_ptr = node as *mut _;

                        if node.is_leaf() {
                            // This node is a leaf, do the swap and return
                            self.top = (node_ptr, 0);
                            node.unsafe_swap(0, &mut *key_ptr, &mut *val_ptr);
                            break;
                        } else {
                            // This node is internal, go deeper
                            self.stack.push((node_ptr, 0));
                            temp_node = node.unsafe_edge_mut(0);
                        }
                    }
                }
            }
        }
    }
}
