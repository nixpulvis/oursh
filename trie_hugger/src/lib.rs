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
        if self.is_empty() {
            0
        } else {
            self.children.iter().fold(1, |m, c| max(m, c.depth() + 1))
        }
    }

    pub fn count(&self) -> usize {
        let mut count = self.children.iter().fold(0, |a, c| a + c.len());
        if self.is_member { count += 1 };
        count
    }

    pub fn insert(&mut self, value: String) {
        print!("adding {} to {:?}: ", value, self);

        for child in self.children.iter_mut() {
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

                        println!("split child")
                        // TODO: Like a lumberjack, split the trie.
                    }
                    // Our value is longer than the child, insert into it.
                    Left(_) => {
                        println!("into child");
                        child.children.push(Trie {
                            is_member: true,
                            value: value[value.len()-i..].into(),
                            children: vec![],
                        });
                        return;
                    }
                    // Our value is shorter than the child, insert this child
                    // onto a new node for the value.
                    Right(_) => {
                        println!("onto child");
                        // TODO: Insert a node for our value, and put this
                        // child in it.
                    }
                }
            }
        }

        // No match in children.
        println!("push");
        self.children.push(Trie {
            is_member: true,
            value: value,
            children: vec![],
        });
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
        assert_eq!(3, trie.len());
    }

    #[test]
    fn insert_one() {
        let mut trie = Trie::default();
        trie.insert("foo".into());
        assert!(! trie.is_empty());
        assert_eq!(trie.depth(), 1);
    }

    #[test]
    fn insert_non_overlapping() {
        let mut trie = Trie::default();
        trie.insert("foo".into());
        trie.insert("bar".into());
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.depth(), 1);
    }

    #[test]
    fn insert_into_child() {
        let mut trie = Trie::default();
        trie.insert("foo".into());
        trie.insert("foobar".into());
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.depth(), 2);
        // TODO: Assert matches closer.
    }

    #[test]
    fn insert_onto_child() {
        let mut trie = Trie::default();
        trie.insert("foobar".into());
        trie.insert("foo".into());
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
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.depth(), 3);
        // TODO: Assert matches closer.
    }
}
