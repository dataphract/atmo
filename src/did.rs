use crate::split_once;

pub struct Did(String);

impl Did {
    pub fn new(did: &[u8]) -> Option<Did> {
        is_valid_did(did).then(|| Did(String::from_utf8(did.into()).unwrap()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn method(&self) -> &str {
        self.0
            .split(':')
            .nth(1)
            .expect("DID missing method, should have been validated already")
    }
}

fn is_valid_did(input: &[u8]) -> bool {
    let Some(input) = input.strip_prefix(b"did:") else {
        return false;
    };

    let Some((method, ident)) = split_once(input, |&c| c == b':') else {
        return false;
    };

    is_valid_did_method(method) && is_valid_did_ident(ident)
}

#[inline]
fn is_valid_did_method(input: &[u8]) -> bool {
    !input.is_empty() && input.iter().all(|b| b.is_ascii_lowercase())
}

fn is_valid_did_ident(input: &[u8]) -> bool {
    if input.is_empty() {
        return false;
    }

    let mut it = input.iter().copied().peekable();

    while let Some(c) = it.next() {
        match c {
            // Accept.
            b'.' | b'_' | b'-' => (),
            a if a.is_ascii_alphanumeric() => (),

            // Identifier cannot end with a colon.
            b':' => {
                if it.peek().is_none() {
                    return false;
                }
            }

            // Percent must be followed by two hex digits.
            b'%' => {
                if !(it.next().is_some_and(|c| c.is_ascii_hexdigit())
                    && it.next().is_some_and(|c| c.is_ascii_hexdigit()))
                {
                    return false;
                }
            }

            // Reject anything else.
            _ => return false,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! valid {
        ($($input:literal),* $(,)?) => {
            $(
                if Did::new($input.as_bytes()).is_none() {
                    panic!("valid DID rejected: {}", $input);
                }
            )*
        };
    }

    macro_rules! invalid {
        ($($input:literal),* $(,)?) => {
            $(
                if Did::new($input.as_bytes()).is_some() {
                    panic!("invalid DID accepted: {}", $input);
                }
            )*
        };
    }

    #[test]
    fn valid_examples() {
        valid![
            "did:plc:z72i7hdynmk6r22z27h6tvur",
            "did:web:blueskyweb.xyz",
            "did:method:val:two",
            "did:m:v",
            "did:method::::val",
            "did:method:-:_:.",
            "did:key:zQ3shZc2QzApp2oymGvQbzP8eKheVshBHbU4ZYjeXqwSKEn6N",
        ];
    }

    #[test]
    fn invalid_examples() {
        invalid![
            "did:METHOD:val",
            "did:m123:val",
            "DID:method:val",
            "did:method:",
            "did:method:val/two",
            "did:method:val?two",
            "did:method:val#two",
        ];
    }
}
