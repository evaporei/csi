/// Get the capacity of a node from the order of the parent B-Tree
fn capacity_from_b(b: usize) -> usize {
    2 * b - 1
}

/// Get the minimum load of a node from its capacity
fn min_load_from_capacity(cap: usize) -> usize {
    // B - 1
    cap / 2
}

use std::ptr;

/// Takes a Vec, and splits half the elements into a new one.
fn split<T>(left: &mut Vec<T>) -> Vec<T> {
    // This function is intended to be called on a full Vec of size 2B - 1 (keys, values),
    // or 2B (edges). In the former case, left should get B elements, and right should get
    // B - 1. In the latter case, both should get B. Therefore, we can just always take the last
    // size / 2 elements from left, and put them on right. This also ensures this method is
    // safe, even if the Vec isn't full. Just uninteresting for our purposes.
    let len = left.len();
    let right_len = len / 2;
    let left_len = len - right_len;
    let mut right = Vec::with_capacity(left.capacity());
    unsafe {
        let left_ptr = &left.as_slice()[left_len] as *const _;
        let right_ptr = right.as_mut_slice().as_mut_ptr();
        ptr::copy_nonoverlapping(left_ptr, right_ptr, right_len);
        left.set_len(left_len);
        right.set_len(right_len);
    }
    right
}

use SearchResult::*;
use std::cmp::Ordering::*;

struct Node<K, V> {
    keys: Vec<K>,
    edges: Vec<Node<K, V>>,
    vals: Vec<V>,
}

use std::mem;

impl<K, V> Node<K, V> {
        /// Make a new internal node
    pub fn new_internal(capacity: usize) -> Node<K, V> {
        Node {
            keys: Vec::with_capacity(capacity),
            vals: Vec::with_capacity(capacity),
            edges: Vec::with_capacity(capacity + 1),
        }
    }

    /// Make a new leaf node
    pub fn new_leaf(capacity: usize) -> Node<K, V> {
        Node {
            keys: Vec::with_capacity(capacity),
            vals: Vec::with_capacity(capacity),
            edges: Vec::new(),
        }
    }

    /// Make a leaf root from scratch
    pub fn make_leaf_root(b: usize) -> Node<K, V> {
        Node::new_leaf(capacity_from_b(b))
    }

    /// Make an internal root and swap it with an old root
    pub fn make_internal_root(left_and_out: &mut Node<K,V>, b: usize, key: K, value: V,
            right: Node<K,V>) {
        let mut node = Node::new_internal(capacity_from_b(b));
        mem::swap(left_and_out, &mut node);
        left_and_out.keys.push(key);
        left_and_out.vals.push(value);
        left_and_out.edges.push(node);
        left_and_out.edges.push(right);
    }


    /// How many key-value pairs the node contains
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// How many key-value pairs the node can fit
    pub fn capacity(&self) -> usize {
        self.keys.capacity()
    }

    /// If the node has any children
    pub fn is_leaf(&self) -> bool {
        self.edges.is_empty()
    }

    /// if the node has too few elements
    pub fn is_underfull(&self) -> bool {
        self.len() < min_load_from_capacity(self.capacity())
    }

    /// if the node cannot fit any more elements
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Swap the given key-value pair with the key-value pair stored in the node's index,
    /// without checking bounds.
    pub unsafe fn unsafe_swap(&mut self, index: usize, key: &mut K, val: &mut V) {
        mem::swap(&mut self.keys.as_mut_slice()[index], key);
        mem::swap(&mut self.vals.as_mut_slice()[index], val);
    }

    /// Get the node's key mutably without any bounds checks.
    pub unsafe fn unsafe_key_mut(&mut self, index: usize) -> &mut K {
        &mut self.keys.as_mut_slice()[index]
    }

    /// Get the node's value at the given index
    pub fn val(&self, index: usize) -> Option<&V> {
        self.vals.as_slice().get(index)
    }

    /// Get the node's value mutably without any bounds checks.
    pub unsafe fn unsafe_val_mut(&mut self, index: usize) -> &mut V {
        &mut self.vals.as_mut_slice()[index]
    }

    /// Get the node's edge at the given index
    pub fn edge(&self, index: usize) -> Option<&Node<K,V>> {
        self.edges.as_slice().get(index)
    }

    /// Get the node's edge mutably at the given index
    pub fn edge_mut(&mut self, index: usize) -> Option<&mut Node<K,V>> {
        self.edges.as_mut_slice().get_mut(index)
    }

    /// Get the node's edge mutably without any bounds checks.
    pub unsafe fn unsafe_edge_mut(&mut self, index: usize) -> &mut Node<K,V> {
        &mut self.edges.as_mut_slice()[index]
    }

    /// Pop an edge off the end of the node
    pub fn pop_edge(&mut self) -> Option<Node<K,V>> {
        self.edges.pop()
    }

    /// Try to insert this key-value pair at the given index in this internal node
    /// If the node is full, we have to split it.
    ///
    /// Returns a *mut V to the inserted value, because the caller may want this when
    /// they're done mutating the tree, but we don't want to borrow anything for now.
    pub fn insert_as_leaf(&mut self, index: usize, key: K, value: V) ->
            (InsertionResult<K, V>, *mut V) {
        if !self.is_full() {
            // The element can fit, just insert it
            self.insert_fit_as_leaf(index, key, value);
            (InsertionResult::Fit, unsafe { self.unsafe_val_mut(index) as *mut _ })
        } else {
            // The element can't fit, this node is full. Split it into two nodes.
            let (new_key, new_val, mut new_right) = self.split();
            let left_len = self.len();

            let ptr = if index <= left_len {
                self.insert_fit_as_leaf(index, key, value);
                unsafe { self.unsafe_val_mut(index) as *mut _ }
            } else {
                new_right.insert_fit_as_leaf(index - left_len - 1, key, value);
                unsafe { new_right.unsafe_val_mut(index - left_len - 1) as *mut _ }
            };

            (InsertionResult::Split(new_key, new_val, new_right), ptr)
        }
    }

    /// Try to insert this key-value pair at the given index in this internal node
    /// If the node is full, we have to split it.
    pub fn insert_as_internal(&mut self, index: usize, key: K, value: V, right: Node<K, V>)
            -> InsertionResult<K, V> {
        if !self.is_full() {
            // The element can fit, just insert it
            self.insert_fit_as_internal(index, key, value, right);
            InsertionResult::Fit
        } else {
            // The element can't fit, this node is full. Split it into two nodes.
            let (new_key, new_val, mut new_right) = self.split();
            let left_len = self.len();

            if index <= left_len {
                self.insert_fit_as_internal(index, key, value, right);
            } else {
                new_right.insert_fit_as_internal(index - left_len - 1, key, value, right);
            }

            InsertionResult::Split(new_key, new_val, new_right)
        }
    }

    /// Remove the key-value pair at the given index
    pub fn remove_as_leaf(&mut self, index: usize) -> (K, V) {
        (self.keys.remove(index), self.vals.remove(index))
    }

    /// Handle an underflow in this node's child. We favour handling "to the left" because we know
    /// we're empty, but our neighbour can be full. Handling to the left means when we choose to
    /// steal, we pop off the end of our neighbour (always fast) and "unshift" ourselves
    /// (always slow, but at least faster since we know we're half-empty).
    /// Handling "to the right" reverses these roles. Of course, we merge whenever possible
    /// because we want dense nodes, and merging is about equal work regardless of direction.
    pub fn handle_underflow(&mut self, underflowed_child_index: usize) {
        assert!(underflowed_child_index <= self.len());
        unsafe {
            if underflowed_child_index > 0 {
                self.handle_underflow_to_left(underflowed_child_index);
            } else {
                self.handle_underflow_to_right(underflowed_child_index);
            }
        }
    }
}

// Private implementation details
impl<K, V> Node<K, V> {
    /// Make a node from its raw components
    fn from_vecs(keys: Vec<K>, vals: Vec<V>, edges: Vec<Node<K, V>>) -> Node<K, V> {
        Node {
            keys,
            vals,
            edges,
        }
    }

    /// We have somehow verified that this key-value pair will fit in this internal node,
    /// so insert under that assumption.
    fn insert_fit_as_leaf(&mut self, index: usize, key: K, val: V) {
        self.keys.insert(index, key);
        self.vals.insert(index, val);
    }

    /// We have somehow verified that this key-value pair will fit in this internal node,
    /// so insert under that assumption
    fn insert_fit_as_internal(&mut self, index: usize, key: K, val: V, right: Node<K, V>) {
        self.keys.insert(index, key);
        self.vals.insert(index, val);
        self.edges.insert(index + 1, right);
    }

    /// Node is full, so split it into two nodes, and yield the middle-most key-value pair
    /// because we have one too many, and our parent now has one too few
    fn split(&mut self) -> (K, V, Node<K, V>) {
        let r_keys = split(&mut self.keys);
        let r_vals = split(&mut self.vals);
        let r_edges = if self.edges.is_empty() {
            Vec::new()
        } else {
            split(&mut self.edges)
        };

        let right = Node::from_vecs(r_keys, r_vals, r_edges);
        // Pop it
        let key = self.keys.pop().unwrap();
        let val = self.vals.pop().unwrap();

        (key, val, right)
    }

    /// Right is underflowed. Try to steal from left,
    /// but merge left and right if left is low too.
    unsafe fn handle_underflow_to_left(&mut self, underflowed_child_index: usize) {
        let left_len = self.edges[underflowed_child_index - 1].len();
        if left_len > min_load_from_capacity(self.capacity()) {
            self.steal_to_left(underflowed_child_index);
        } else {
            self.merge_children(underflowed_child_index - 1);
        }
    }

    /// Left is underflowed. Try to steal from the right,
    /// but merge left and right if right is low too.
    unsafe fn handle_underflow_to_right(&mut self, underflowed_child_index: usize) {
        let right_len = self.edges[underflowed_child_index + 1].len();
        if right_len > min_load_from_capacity(self.capacity()) {
            self.steal_to_right(underflowed_child_index);
        } else {
            self.merge_children(underflowed_child_index);
        }
    }

    /// Steal! Stealing is roughly analagous to a binary tree rotation.
    /// In this case, we're "rotating" right.
    unsafe fn steal_to_left(&mut self, underflowed_child_index: usize) {
        // Take the biggest stuff off left
        let (mut key, mut val, edge) = {
            let left = self.unsafe_edge_mut(underflowed_child_index - 1);
            match (left.keys.pop(), left.vals.pop(), left.edges.pop()) {
                (Some(k), Some(v), e) => (k, v, e),
                _ => unreachable!(),
            }
        };

        // Swap the parent's seperating key-value pair with left's
        self.unsafe_swap(underflowed_child_index - 1, &mut key, &mut val);

        // Put them at the start of right
        {
            let right = self.unsafe_edge_mut(underflowed_child_index);
            right.keys.insert(0, key);
            right.vals.insert(0, val);
            match edge {
                None => {}
                Some(e) => right.edges.insert(0, e)
            }
        }
    }

    /// Steal! Stealing is roughly analagous to a binary tree rotation.
    /// In this case, we're "rotating" left.
    unsafe fn steal_to_right(&mut self, underflowed_child_index: usize) {
        // Take the smallest stuff off right
        let (mut key, mut val, edge) = {
            let right = self.unsafe_edge_mut(underflowed_child_index + 1);
            (right.keys.remove(0), right.vals.remove(0), right.edges.remove(0))
        };

        // Swap the parent's seperating key-value pair with right's
        self.unsafe_swap(underflowed_child_index, &mut key, &mut val);

        // Put them at the end of left
        {
            let left = self.unsafe_edge_mut(underflowed_child_index);
            left.keys.push(key);
            left.vals.push(val);
            left.edges.push(edge);
        }
    }

    /// Merge! Left and right will be smooshed into one node, along with the key-value
    /// pair that seperated them in their parent.
    unsafe fn merge_children(&mut self, left_index: usize) {
        // Permanently remove right's index, and the key-value pair that seperates
        // left and right
        let (key, val, right) = {
            (self.keys.remove(left_index),
                self.vals.remove(left_index),
                self.edges.remove(left_index + 1))
        };

        // Give left right's stuff.
        let left = self.unsafe_edge_mut(left_index);
        left.absorb(key, val, right);
    }

    /// Take all the values from right, seperated by the given key and value
    fn absorb(&mut self, key: K, val: V, right: Node<K, V>) {
        // Just as a sanity check, make sure we can fit this guy in
        debug_assert!(self.len() + right.len() <= self.capacity());

        self.keys.push(key);
        self.vals.push(val);
        self.keys.extend(right.keys.into_iter());
        self.vals.extend(right.vals.into_iter());
        self.edges.extend(right.edges.into_iter());
    }
}

impl<K: Ord, V> Node<K, V> {
    // for now linear
    fn search<Q: ?Sized>(&self, key: &Q) -> SearchResult
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        for (i, k) in self.keys.iter().enumerate() {
            match k.borrow().cmp(key) {
                Less => {},
                Equal => return Found(i),
                Greater => return GoDown(i),
            }
        }
        GoDown(self.len())
    }
}

pub struct BTreeMap<K, V> {
    root: Node<K, V>,
    length: usize,
    depth: usize,
    b: usize,
}

impl<K, V> BTreeMap<K, V> {
    pub fn new() -> Self {
        Self::with_b(6)
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

enum SearchResult {
    Found(usize),
    GoDown(usize),
}

enum InsertionResult<K, V> {
    /// The inserted element fit
    Fit,
    /// The inserted element did not fit, so the node was split
    Split(K, V, Node<K, V>),
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
                },
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
                }
                GoDown(i) => {
                    stack = match stack.push(i) {
                        PushResult::Done(new_stack) => {
                            new_stack.insert(key, value);
                            return None;
                        }
                        PushResult::Grew(new_stack) => {
                            new_stack
                        }
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        use stack::*;

        let mut stack = PartialSearchStack::new(self);
        loop {
            match stack.next().search(key) {
                Found(i) => {
                    return Some(stack.seal(i).remove());
                }
                GoDown(i) => {
                    stack = match stack.push(i) {
                        PushResult::Done(_) => return None,
                        PushResult::Grew(new_stack) => {
                            new_stack
                        }
                    };
                }
            }
        }
    }
}

mod stack {
    use super::BTreeMap;
    use super::*;

    type StackItem<K, V> = (*mut Node<K, V>, usize);
    type Stack<K, V> = Vec<StackItem<K, V>>;

    /// A PartialSearchStack handles the construction of a search stack.
    pub struct PartialSearchStack<'a, K:'a, V:'a> {
        map: &'a mut BTreeMap<K, V>,
        stack: Stack<K, V>,
        next: *mut Node<K, V>,
    }

    /// A SearchStack represents a full path to an element of interest. It provides methods
    /// for manipulating the element at the top of its stack.
    pub struct SearchStack<'a, K:'a, V:'a> {
        map: &'a mut BTreeMap<K, V>,
        stack: Stack<K, V>,
        top: StackItem<K, V>,
    }

    /// The result of asking a PartialSearchStack to push another node onto itself. Either it
    /// Grew, in which case it's still Partial, or it found its last node was actually a leaf, in
    /// which case it seals itself and yields a complete SearchStack.
    pub enum PushResult<'a, K:'a, V:'a> {
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
            let next_node = unsafe {
                &mut *next_ptr
            };
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
                },
            }
        }

        /// Converts the stack into a mutable reference to its top.
        pub fn into_next(self) -> &'a mut Node<K, V> {
            unsafe {
                &mut *self.next
            }
        }

        /// Gets the top of the stack.
        pub fn next(&self) -> &Node<K, V> {
            unsafe {
                &*self.next
            }
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
                let (mut insertion, inserted_ptr) = {
                    (*node).insert_as_leaf(index, key, val)
                };

                use InsertionResult::*;

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
                        }
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
                    (node.unsafe_key_mut(index) as *mut _,
                     node.unsafe_val_mut(index) as *mut _)
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
}
