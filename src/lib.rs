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
    tokens: std::vec::Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            chars: input.chars(),
            tokens: Vec::new(),
        }
    }

    pub fn match_tokens(&mut self) -> () {
        let next = self.chars.next();
        self.match_any_char(next);
    }

    fn match_any_char(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                let next = self.chars.next();
                self.match_assign(next);
            }
            Some(' ') => {
                self.tokens.push(Whitespace);
                let next = self.chars.next();
                self.match_any_char(next);
            }
            None => self.tokens.push(Eof),
            _ => {
                self.tokens.push(Unknown);
                let next = self.chars.next();
                self.match_any_char(next);
            }
        }
    }

    fn match_assign(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                let next = self.chars.next();
                return self.match_eq(next);
            }
            Some('>') => {
                self.tokens.push(Arrow);
                let next = self.chars.next();
                self.match_any_char(next);
            }
            _ => {
                self.tokens.push(Assign);
                self.match_any_char(c);
            }
        }
    }

    fn match_eq(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                self.tokens.push(EqStrict);
                let next = self.chars.next();
                self.match_any_char(next);
            }
            _ => {
                self.tokens.push(Eq);
                self.match_any_char(c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign() {
        let mut lexer = Lexer::new("=");
        lexer.match_tokens();
        assert_eq!(lexer.tokens, vec![Assign, Eof]);
    }

    #[test]
    fn arrow() {
        let mut lexer = Lexer::new("=>");
        lexer.match_tokens();
        assert_eq!(lexer.tokens, vec![Arrow, Eof]);
    }

    #[test]
    fn eq() {
        let mut lexer = Lexer::new("==");
        lexer.match_tokens();
        assert_eq!(lexer.tokens, vec![Eq, Eof]);
    }

    #[test]
    fn eq_strict() {
        let mut lexer = Lexer::new("===");
        lexer.match_tokens();
        assert_eq!(lexer.tokens, vec![EqStrict, Eof]);
    }

    #[test]
    fn unknown() {
        let mut lexer = Lexer::new("_");
        lexer.match_tokens();
        assert_eq!(lexer.tokens, vec![Unknown, Eof]);
    }

    #[test]
    fn eof() {
        let mut lexer = Lexer::new("");
        lexer.match_tokens();
        assert_eq!(lexer.tokens, vec![Eof]);
    }

    #[test]
    fn whitespace() {
        let mut lexer = Lexer::new(" ");
        lexer.match_tokens();
        assert_eq!(lexer.tokens, vec![Whitespace, Eof]);
    }

    #[test]
    fn assign_and_whitespace() {
        let mut lexer = Lexer::new("= ");
        lexer.match_tokens();
        assert_eq!(lexer.tokens, vec![Assign, Whitespace, Eof]);
    }
}
