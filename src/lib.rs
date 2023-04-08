#[derive(PartialEq, Debug)]
pub enum Token {
    Assign,   // =
    Arrow,    // =>
    Eq,       // ==
    EqStrict, // ===
    Eof,      // End of file
    Unknown,  // Fallback for unhandled / unexpected / unknown input
}

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            chars: input.chars(),
        }
    }

    pub fn get_token(&mut self) -> Token {
        use Token::*;

        match self.chars.next() {
            Some('=') => match self.chars.next() {
                Some('=') => match self.chars.next() {
                    Some('=') => EqStrict,
                    _ => Eq,
                },
                Some('>') => Arrow,
                _ => Assign,
            },
            None => Eof,
            _ => Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;

    #[test]
    fn assign() {
        assert_eq!(Lexer::new("=").get_token(), Assign);
    }

    #[test]
    fn arrow() {
        assert_eq!(Lexer::new("=>").get_token(), Arrow);
    }

    #[test]
    fn eq() {
        assert_eq!(Lexer::new("==").get_token(), Eq);
    }

    #[test]
    fn eq_strict() {
        assert_eq!(Lexer::new("===").get_token(), EqStrict);
    }

    #[test]
    fn unknown() {
        assert_eq!(Lexer::new("_").get_token(), Unknown);
    }

    #[test]
    fn eof() {
        assert_eq!(Lexer::new("").get_token(), Eof);
    }
}
