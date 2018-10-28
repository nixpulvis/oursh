use std::cmp::max;

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

    pub fn len(&self) -> usize {
        let mut count = self.children.iter().fold(0, |a, c| a + c.len());
        if self.is_member { count += 1 };
        count
    }

    pub fn depth(&self) -> usize {
        let base = if self.is_empty() { 0 } else { 1 };
        self.children.iter().fold(base, |m, c| max(m, c.depth()))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, value: String) {
        // println!("adding {} to {:?}", value, self);

        for child in self.children.iter() {
            // if the value starts with the child node's value then
            // insert the whole thing into that child with the child's
            // value removed.
            //
            // TODO: if the child node's value starts with the value
            // we're inserting, we need to insert our node above the
            // child node and insert the child node into us. All the
            // children below shouldn't need to be updated.
            if value.starts_with(child.value()) {
                // TODO: Insert into child and return.
            }
        }

        // No match in children.
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
}
