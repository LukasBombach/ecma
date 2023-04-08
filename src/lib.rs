#[derive(PartialEq, Debug)]
pub enum Token {
    Assign,     // =
    Arrow,      // =>
    Eq,         // ==
    EqStrict,   // ===
    Whitespace, // Any whitespace except newlines, "spacebar", tabs etc
    Eof,        // End of file
    Unknown,    // Fallback for unhandled / unexpected / unknown input
}

use Token::*;

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
        match self.chars.next() {
            Some('=') => match self.chars.next() {
                Some('=') => match self.chars.next() {
                    Some('=') => EqStrict,
                    _ => Eq,
                },
                Some('>') => Arrow,
                _ => Assign,
            },
            Some(' ') => Whitespace,
            None => Eof,
            _ => Unknown,
        }
    }

    fn match_any_char(&mut self, c: Option<char>) -> Token {
        match c {
            Some('=') => {
                let next = self.chars.next();
                return self.match_assign(next);
            }
            Some(' ') => Whitespace,
            None => Eof,
            _ => Unknown,
        }
    }

    fn match_assign(&mut self, c: Option<char>) -> Token {
        match c {
            Some('=') => {
                let next = self.chars.next();
                return self.match_eq(next);
            }
            Some('>') => Arrow,
            _ => Assign,
        }
    }

    fn match_eq(&mut self, c: Option<char>) -> Token {
        match c {
            Some('=') => EqStrict,
            _ => Eq,
        }
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn whitespace() {
        assert_eq!(Lexer::new(" ").get_token(), Whitespace);
    }

    #[test]
    fn assign_and_whitespace() {
        let mut lexer = Lexer::new("= ");
        assert_eq!(lexer.get_token(), Assign);
        assert_eq!(lexer.get_token(), Whitespace);
    }
}
