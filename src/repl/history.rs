//! Keeps a record of previous commands, used for completion and archeology.
use std::{
    env,
    io::prelude::*,
    fs::File,
    path::Path,
};

/// The history of a user's provided commands.
#[derive(Debug)]
pub struct History(pub Option<usize>, pub Vec<(String, usize)>);

impl History {
    pub fn reset_index(&mut self) {
        self.0 = None;
    }

    pub fn add(&mut self, text: &str, count: usize) {
        if text.is_empty() {
            return;
        }

        // HACK: There's got to be a cleaner way.
        let mut index = 0;
        if self.1.iter().enumerate().find(|(i, (t, _))| {
            index = *i;
            text == t
        }).is_some() {
            self.1[index].1 += count;
            let text = self.1.remove(index);
            self.1.insert(0, text);
            debug!("updating history item: {:?}", self.1[0]);
        } else {
            self.1.insert(0, (text.to_owned(), count));
            debug!("adding history item: {:?}", self.1[0]);
        }
    }

    pub fn get_up(&mut self) -> Option<String> {
        let text_len = self.1.len();
        if text_len > 0 {
            match self.0 {
                Some(i) => {
                    self.0 = Some(i.saturating_add(1)
                                   .min(text_len - 1));
                },
                None => self.0 = Some(0),
            }
        } else {
            self.0 = None;
        }

        match self.0 {
            Some(i) => Some(self.1[i].0.clone()),
            None => None,
        }
    }

    pub fn get_down(&mut self) -> Option<String> {
        match self.0 {
            Some(i) if i == 0 => self.0 = None,
            Some(i) => self.0 = Some(i.saturating_sub(1)),
            None => {},
        };

        match self.0 {
            Some(i) => Some(self.1[i].0.clone()),
            None => None,
        }
    }

    pub fn load() -> Self {
        let mut history = History(None, vec![]);
        let home = env::var("HOME").expect("HOME variable not set.");
        let history_path = format!("{}/.oursh_history", home);
        if Path::new(&history_path).exists() {
            let mut f = File::open(&history_path)
                .expect("error cannot find history");
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("error reading history");
            // TODO: We really need something like serde or serde-json
            //       for the pair if we want to have historical run counts.
            // let hist = contents.split("\n").map(|s| {
            //     String::from(s).split(" ").map(|s| {
            //         println!("{:?}", s);
            //     })
            // }).collect::<Vec<String, usize>>();
            let hist = contents.split('\n').map(|s| {
                (String::from(s), 0)
            });

            // Add each entry to the history in order.
            for (text, index) in hist {
                history.add(&text, index);
            }

            // Reverse the order so users get the most recent commands first.
            history.1 = history.1.into_iter().rev().collect();
        }

        history
    }

    pub fn save(&self) -> Result<(), ()> {
        let home = env::var("HOME").expect("HOME variable not set.");
        let history_path = format!("{}/.oursh_history", home);
        let mut f = File::create(&history_path)
            .expect("error cannot find history");
        for (text, _) in self.1.iter() {
            f.write_all(text.as_bytes())
                .expect("error writing history");
            f.write_all(b"\n")
                .expect("error writing history");
        }

        Ok(())
    }
}
