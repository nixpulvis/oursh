use std::env;
use std::io::Write;
use nix::unistd;
use crate::{NAME, VERSION};

/// TODO: docs
pub fn ps1(stdout: &mut impl Write) {
    let prompt = expand_prompt(env::var("PS1").unwrap_or_else(|_| "\\s-\\v\\$ ".into()));
    write!(stdout, "{}", prompt).unwrap();
    stdout.flush().unwrap();
}

fn expand_prompt(prompt: String) -> String {
    let mut result = String::new();
    let mut command = false;
    let mut octal = vec![];
    for c in prompt.chars() {
        let o = octal.iter().map(|c: &char| c.to_string())
                     .collect::<Vec<_>>()
                     .join("");
        if !octal.is_empty() && octal.len() < 3 {
            if ('0'..'8').contains(&c) {
                octal.push(c);
            } else {
                result += &o;
                octal.clear();
            }
        } else if octal.len() == 3 {
            if let Ok(n) = u8::from_str_radix(&o, 8) {
                result.push(n as char);
            }
            octal.clear();
        }

        if command {
            // TODO: https://ss64.com/bash/syntax-prompt.html
            result += &match c {
                'h' => {
                    let mut buf = [0u8; 64];
                    let cstr = unistd::gethostname(&mut buf).expect("error getting hostname");
                    cstr.to_str().expect("error invalid UTF-8").into()
                }
                'e' => (0x1b as char).into(),
                'u' => env::var("USER").unwrap_or_else(|_| "".into()),
                'w' => env::var("PWD").unwrap_or_else(|_| "".into()),
                's' => NAME.into(),
                'v' => VERSION[0..(VERSION.len() - 2)].into(),
                '0' => { octal.push(c); "".into() },
                '\\' => "".into(),
                c => c.into(),
            };
            command = false;
        } else if c == '\\' {
            command = true;
        } else if octal.is_empty() {
            result.push(c);
        }
    }
    result
}
