use std::cmp::max;
use itertools::{
    Itertools,
    EitherOrBoth::{Both, Left, Right},
};

#[derive(Eq, PartialEq, Debug)]
struct Trie {
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
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn len(&self) -> usize {
        self.children.iter().fold(self.children.len(), |a, c| a + c.len())
    }

    pub fn depth(&self) -> usize {
        self.children.iter().fold(0, |m, c| max(m, c.depth() + 1))
    }

    pub fn count(&self) -> usize {
        let base = if self.is_member { 1 } else { 0 };
        self.children.iter().fold(base, |a, c| a + c.count())
    }

    pub fn insert(&mut self, value: &str) {
        for (n, child) in self.children.iter_mut().enumerate() {
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
                        if i == 0 { break; }

                        self.split(value, n, i);
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
                        self.onto(value, n, i);
                        return;
                    }
                }
            }
        }

        // No match in children.
        self.children.push(Trie {
            is_member: true,
            value: value.into(),
            children: vec![],
        });
    }

    fn split(&mut self, value: &str, n: usize, i: usize) {
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

    fn onto(&mut self, value: &str, n: usize, i: usize) {
        let mut old = self.children.remove(n);
        old.value = old.value[i..].into();
        let node = Trie {
            is_member: true,
            value: value[..i].into(),
            children: vec![old],
        };
        self.children.push(node);
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
}
