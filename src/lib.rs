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
        lexer.match_next();
        lexer.tokens
    }
}

impl<'a> Lexer<'a> {
    fn push_and_match(&mut self, token: Token, c: Option<char>) -> () {
        self.tokens.push(token);
        self.match_char(c);
    }

    fn push_and_next(&mut self, token: Token) -> () {
        self.tokens.push(token);
        self.match_next();
    }
}

impl<'a> Lexer<'a> {
    fn match_next(&mut self) -> () {
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
                self.push_and_next(Whitespace);
            }
            None => self.tokens.push(Eof),
            _ => {
                self.push_and_next(Unknown);
            }
        }
    }
}

impl<'a> Lexer<'a> {
    fn match_assign(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                let next = self.chars.next();
                self.match_eq(next);
            }
            Some('>') => {
                self.push_and_next(Arrow);
            }
            _ => {
                self.push_and_match(Assign, c);
            }
        }
    }

    fn match_eq(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                self.push_and_next(EqStrict);
            }
            _ => {
                self.push_and_match(Eq, c);
            }
        }
    }
}

impl<'a> Lexer<'a> {
    fn match_lt(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                self.push_and_next(Lte);
            }
            Some('<') => {
                let next = self.chars.next();
                self.match_shl(next);
            }
            _ => {
                self.push_and_match(Lt, c);
            }
        }
    }

    fn match_shl(&mut self, c: Option<char>) -> () {
        match c {
            Some('=') => {
                self.push_and_next(ShlAssign);
            }
            _ => {
                self.push_and_match(Shl, c);
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
