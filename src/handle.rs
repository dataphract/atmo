use crate::{is_valid_domain_segment, is_valid_tld};

const MAX_LEN: usize = 253;

pub struct Handle(String);

impl Handle {
    pub fn new(handle: &str) -> Option<Handle> {
        if handle.len() > MAX_LEN {
            return None;
        }

        let bytes = handle.as_bytes();

        let mut it = bytes.split(|&b| b == b'.').peekable();

        let mut num_segments = 0;
        while let Some(segment) = it.next() {
            num_segments += 1;

            let is_valid = match it.peek() {
                Some(_) => is_valid_domain_segment(segment),
                None => is_valid_tld(segment),
            };

            if !is_valid {
                return None;
            }
        }

        if num_segments < 2 {
            return None;
        }

        Some(Handle(handle.to_ascii_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! valid {
        ($($input:literal),* $(,)?) => {
            $(
                if Handle::new($input.into()).is_none() {
                    panic!("valid Handle rejected: {}", $input);
                }
            )*
        };
    }

    macro_rules! invalid {
        ($($input:literal),* $(,)?) => {
            $(
                if Handle::new($input.into()).is_some() {
                    panic!("invalid Handle accepted: {}", $input);
                }
            )*
        };
    }

    #[test]
    fn valid_examples() {
        valid![
            "jay.bsky.social",
            "8.cn",
            "name.t--t",
            "XX.LCS.MIT.EDU",
            "a.co",
            "xn--notarealidn.com",
            "xn--fiqa61au8b7zsevnm8ak20mc4a87e.xn--fiqs8s",
            "xn--ls8h.test",
            "example.t",
        ];
    }

    #[test]
    fn invalid_examples() {
        invalid![
            "",
            "jo@hn.test",
            "💩.test",
            "john..test",
            "xn--bcher-.tld",
            "john.0",
            "cn.8",
            "www.masełkowski.pl.com",
            "org",
            "name.org.",
        ];
    }
}
