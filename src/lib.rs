#[derive(PartialEq, Debug)]
pub enum Token {
    Assign,   // =
    Arrow,    // =>
    Eq,       // ==
    EqStrict, // ===

    Lt,        // <
    Lte,       // <=
    Shl,       // <<
    ShlAssign, // <<=

    Whitespace, // Any whitespace except newlines, "spacebar", tabs etc

    Eof,     // End of file
    Unknown, // Fallback for unhandled / unexpected / unknown input
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

    pub fn get_tokens(input: &'a str) -> std::vec::Vec<Token> {
        let mut lexer = Lexer::new(input);
        lexer.match_next_char();
        lexer.tokens
    }
}

impl<'a> Lexer<'a> {
    fn match_next_char(&mut self) -> () {
        let next = self.chars.next();
        self.match_char(next);
    }

    fn match_char(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                let next = self.chars.next();
                self.match_assign(next);
            }
            Some('<') => {
                let next = self.chars.next();
                self.match_lt(next);
            }
            Some(' ') => {
                self.tokens.push(Whitespace);
                self.match_next_char();
            }
            None => self.tokens.push(Eof),
            _ => {
                self.tokens.push(Unknown);
                self.match_next_char();
            }
        }
    }
}

impl<'a> Lexer<'a> {
    fn match_assign(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                let next = self.chars.next();
                return self.match_eq(next);
            }
            Some('>') => {
                self.tokens.push(Arrow);
                self.match_next_char();
            }
            _ => {
                self.tokens.push(Assign);
                self.match_char(c);
            }
        }
    }

    fn match_eq(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                self.tokens.push(EqStrict);
                self.match_next_char();
            }
            _ => {
                self.tokens.push(Eq);
                self.match_char(c);
            }
        }
    }
}

impl<'a> Lexer<'a> {
    fn match_lt(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                self.tokens.push(Lte);
                self.match_next_char();
            }
            Some('<') => {
                let next = self.chars.next();
                return self.match_shl(next);
            }
            _ => {
                self.tokens.push(Lt);
                self.match_char(c);
            }
        }
    }

    fn match_shl(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                self.tokens.push(ShlAssign);
                self.match_next_char();
            }
            _ => {
                self.tokens.push(Shl);
                self.match_char(c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign() {
        assert_eq!(Lexer::get_tokens("="), vec![Assign, Eof]);
    }

    #[test]
    fn arrow() {
        assert_eq!(Lexer::get_tokens("=>"), vec![Arrow, Eof]);
    }

    #[test]
    fn eq() {
        assert_eq!(Lexer::get_tokens("=="), vec![Eq, Eof]);
    }

    #[test]
    fn eq_strict() {
        assert_eq!(Lexer::get_tokens("==="), vec![EqStrict, Eof]);
    }

    #[test]
    fn unknown() {
        assert_eq!(Lexer::get_tokens("ä"), vec![Unknown, Eof]);
    }

    #[test]
    fn eof() {
        assert_eq!(Lexer::get_tokens(""), vec![Eof]);
    }

    #[test]
    fn whitespace() {
        assert_eq!(Lexer::get_tokens(" "), vec![Whitespace, Eof]);
    }

    #[test]
    fn assign_and_whitespace() {
        assert_eq!(Lexer::get_tokens("= "), vec![Assign, Whitespace, Eof]);
    }

    #[test]
    fn multiple_tokens_of_different_lengths() {
        assert_eq!(
            Lexer::get_tokens("=> <<= = <<"),
            vec![Arrow, Whitespace, ShlAssign, Whitespace, Assign, Whitespace, Shl, Eof]
        );
    }

    #[test]
    fn lt() {
        assert_eq!(Lexer::get_tokens("<"), vec![Lt, Eof]);
    }

    #[test]
    fn lte() {
        assert_eq!(Lexer::get_tokens("<="), vec![Lte, Eof]);
    }

    #[test]
    fn shl() {
        assert_eq!(Lexer::get_tokens("<<"), vec![Shl, Eof]);
    }

    #[test]
    fn shlassign() {
        assert_eq!(Lexer::get_tokens("<<="), vec![ShlAssign, Eof]);
    }
}
