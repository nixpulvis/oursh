use std::cmp::max;
use itertools::{
    Itertools,
    EitherOrBoth::{Both, Left, Right},
};

#[derive(Eq, PartialEq, Debug)]
pub struct Trie {
    // TODO: Make Option.
    // a Some(true) root "" node will match everything, but still
    // contain, and iterate the inserted values.
    is_member: bool,
    value: String,
    children: Vec<Trie>,
}

impl Default for Trie {
    fn default() -> Self {
        Trie {
            is_member: false,
            value: "".into(),
            children: vec![],
        }
    }
}

impl Trie {
    fn value(&self) -> &str {
        &self.value
    }

    /// Returns true when there are no elements inserted into this trie.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Returns the number of elements inserted into this trie.
    pub fn count(&self) -> usize {
        let base = if self.is_member { 1 } else { 0 };
        self.children.iter().fold(base, |a, c| a + c.count())
    }

    /// Returns the maximum depth of this tree, where a tree with no children
    /// is depth 0.
    pub fn depth(&self) -> usize {
        self.children.iter().fold(0, |m, c| max(m, c.depth() + 1))
    }

    fn len(&self) -> usize {
        self.children.iter().fold(self.children.len(), |a, c| a + c.len())
    }

    pub fn contains(&self, value: &str) -> bool {
        unimplemented!();
    }

    pub fn prefix(&self, value: &str) -> Self {
        unimplemented!();
    }

    /// Insert a string into this trie.
    ///
    /// This function upholds the invariants of this data structure by
    /// potentially shuffling around children when creating new nodes.
    pub fn insert(&mut self, value: &str) {
        'c: for (n, child) in self.children.iter_mut().enumerate() {
            let mut iter = value.chars()
                                .zip_longest(child.value().chars())
                                .enumerate();
            for (i, either) in iter {
                match either {
                    // We're still in the matching part of the two values.
                    Both(v, c) if v == c => continue,
                    // Both values are different.
                    Both(v, c) => {
                        // If we're still at index 0 we don't match at all.
                        if i == 0 { continue 'c; }

                        self.split_node(value, n, i);
                        return;
                    }
                    // Our value is longer than the child, insert into it.
                    Left(_) => {
                        child.insert(&value[i..]);
                        return;
                    }
                    // Our value is shorter than the child, insert this child
                    // onto a new node for the value.
                    Right(_) => {
                        self.inject_node(value, n, i);
                        return;
                    }
                }
            }

            // If we've made it here we have inserted a duplicate, we should
            // ensure it's a member, and return early to prevent inserting
            // another new node.
            child.is_member = true;
            return;
        }

        // No match in children.
        self.new_node(value);
    }

    fn new_node(&mut self, value: &str) {
        self.children.push(Trie {
            is_member: true,
            value: value.into(),
            children: vec![],
        });
    }

    fn split_node(&mut self, value: &str, n: usize, i: usize) {
        let new = Trie {
            is_member: true,
            value: value[i..].into(),
            children: vec![],
        };
        let mut old = self.children.remove(n);
        old.value = old.value[i..].into();
        let node = Trie {
            is_member: false,
            value: value[..i].into(),
            children: vec![old, new],
        };
        self.children.push(node);
    }

    fn inject_node(&mut self, value: &str, n: usize, i: usize) {
        let mut old = self.children.remove(n);
        old.value = old.value[i..].into();
        let node = Trie {
            is_member: true,
            value: value[..i].into(),
            children: vec![old],
        };
        self.children.push(node);
    }

    pub fn remove(&mut self, prefix: &str) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let trie = Trie::default();
        assert!(trie.is_empty());
        assert_eq!(trie.depth(), 0);
    }

    #[test]
    fn len() {
        let mut trie = Trie::default();
        assert_eq!(0, trie.len());
        trie.insert("ba".into());
        assert_eq!(1, trie.len());
        trie.insert("foo".into());
        assert_eq!(2, trie.len());
        trie.insert("bar".into());
        assert_eq!(3, trie.len());
        trie.insert("baz".into());
        assert_eq!(4, trie.len());
        trie.insert("bat".into());
        assert_eq!(5, trie.len());
    }

    #[test]
    fn depth() {
        let mut trie = Trie::default();
        assert_eq!(0, trie.depth());
        trie.insert("ba".into());
        assert_eq!(1, trie.depth());
        trie.insert("bar".into());
        assert_eq!(2, trie.depth());
        trie.insert("baz".into());
        assert_eq!(2, trie.depth());
        trie.insert("foo".into());
        assert_eq!(2, trie.depth());
    }

    #[test]
    fn insert_one() {
        let mut trie = Trie::default();
        trie.insert("foo".into());
        assert_eq!(trie.count(), 1);
        assert!(! trie.is_empty());
        assert_eq!(trie.depth(), 1);
    }

    #[test]
    fn insert_non_overlapping() {
        let mut trie = Trie::default();
        trie.insert("foo".into());
        trie.insert("bar".into());
        assert_eq!(trie.count(), 2);
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.depth(), 1);
    }

    #[test]
    fn insert_into_child() {
        let mut trie = Trie::default();
        trie.insert("foo".into());
        trie.insert("foobar".into());
        assert_eq!(trie.count(), 2);
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.depth(), 2);
        // TODO: Assert matches closer.
    }

    #[test]
    fn insert_onto_child() {
        let mut trie = Trie::default();
        trie.insert("foobar".into());
        trie.insert("foo".into());
        assert_eq!(trie.count(), 2);
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.depth(), 2);
        // TODO: Assert matches closer.
    }

    #[test]
    fn insert_child() {
        let mut into = Trie::default();
        into.insert("foo".into());
        into.insert("foobar".into());
        let mut onto = Trie::default();
        onto.insert("foobar".into());
        onto.insert("foo".into());
        assert_eq!(into, onto);
    }

    #[test]
    fn insert_split_child() {
        let mut trie = Trie::default();
        trie.insert("foobar".into());
        trie.insert("food".into());
        assert_eq!(trie.count(), 2);
        assert_eq!(trie.len(), 3);
        assert_eq!(trie.depth(), 2);
        // TODO: Assert matches closer.
    }

    #[test]
    fn insert_duplicate() {
        let mut trie = Trie::default();
        trie.insert("food".into());
        trie.insert("foobar".into());
        trie.insert("foo".into());
        assert_eq!(trie.count(), 3);
        assert_eq!(trie.len(), 3);
        assert_eq!(trie.depth(), 2);
        // TODO: Assert matches closer.
    }

    // #[test]
    // fn prefix() {
    //     let mut trie = Trie::default();
    //     trie.insert("freedom");
    //     trie.insert("flip");
    //     trie.insert("freak");
    //     assert_eq!(trie.prefix("").count(), 3);
    //     assert_eq!(trie.prefix("fr").count(), 2);
    // }
}
