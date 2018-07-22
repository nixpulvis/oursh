use std::io::Read;

struct Program {
    source: String,
}

impl Program {
    fn parse<R: Read>(mut reader: R) -> Self {
        let mut source = String::new();
        reader.read_to_string(&mut source).expect("TODO");

        Program {
            source: source,
        }
    }

    fn is_valid(&self) -> bool {
        // TODO: check syntax.
        // TODO: check valid command?
        !(self.source.len() == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let program = Program::parse(b"date" as &[u8]);
        assert!(program.is_valid());
    }

    #[test]
    fn test_parse_empty() {
        let program = Program::parse(b"" as &[u8]);
        assert!(!program.is_valid());
    }
}
