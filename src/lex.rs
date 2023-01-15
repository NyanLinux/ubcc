#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Plus,
    Minus,
    Slash,
    Asterisk,
    LParen,
    RParen,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,
    Not,
    Assignment,
    Integer(i32),
    Identifier(String),
    Return,
    If,
    Else,
    While,
    For,
    SemiColon,
    Eof,
}

pub(crate) struct Lexer {
    input: String,
    position: usize,
    consume_position: usize,
    ch: char,
}

impl Lexer {
    pub(crate) fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            consume_position: 0,
            ch: '\0',
        };
        lexer.consume_char();
        lexer
    }

    pub(crate) fn next(&mut self) -> Token {
        self.skip_whitespace();
        match self.ch {
            '+' => {
                self.consume_char();
                Token::Plus
            }
            '-' => {
                self.consume_char();
                Token::Minus
            }
            '*' => {
                self.consume_char();
                Token::Asterisk
            }
            '/' => {
                self.consume_char();
                Token::Slash
            }
            '(' => {
                self.consume_char();
                Token::LParen
            }
            ')' => {
                self.consume_char();
                Token::RParen
            }
            '\0' => {
                self.consume_char();
                Token::Eof
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.consume_char();
                    self.consume_char();
                    Token::NotEq
                } else {
                    self.consume_char();
                    Token::Not
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.consume_char();
                    self.consume_char();
                    Token::Eq
                } else {
                    self.consume_char();
                    Token::Assignment
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.consume_char();
                    self.consume_char();
                    Token::LtEq
                } else {
                    self.consume_char();
                    Token::Lt
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.consume_char();
                    self.consume_char();
                    Token::GtEq
                } else {
                    self.consume_char();
                    Token::Gt
                }
            }
            ';' => {
                self.consume_char();
                Token::SemiColon
            }
            _ => {
                if self.ch.is_numeric() {
                    let num = self.consume_number();
                    Token::Integer(num)
                } else if self.ch >= 'a' && self.ch <= 'z' {
                    let w = self.consume_word();
                    self.word_into_token(w)
                } else {
                    self.report_error("invalid token");
                    panic!();
                }
            }
        }
    }

    fn consume_char(&mut self) {
        if self.consume_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.consume_position).unwrap();
        }
        self.position = self.consume_position;
        self.consume_position += 1;
    }

    fn consume_number(&mut self) -> i32 {
        let position = self.position;
        while self.ch.is_numeric() {
            self.consume_char();
        }
        self.input[position..self.position].parse().unwrap()
    }

    fn consume_word(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.consume_char();
        }
        self.input[position..self.position].to_string()
    }

    fn word_into_token(&self, word: String) -> Token {
        match &*word {
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            _ => Token::Identifier(word),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.consume_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.consume_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.consume_position).unwrap()
        }
    }

    fn report_error(&self, message: &str) {
        let mut error = String::new();
        error.push_str("\x1b[31merror\x1b[0m: ");
        error.push_str(message);
        error.push_str("\n");
        error.push_str(&self.input);
        error.push_str("\n");
        error.push_str(&" ".repeat(self.position));
        error.push_str("\x1b[33m^\x1b[0m\n");
        println!("{}", error);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        {
            let input = String::from(
                "1 + - / * ( ) = ! == != < > <= >= ; a b foo bar return if else while for",
            );
            let mut lexer = Lexer::new(input);
            assert_eq!(lexer.next(), Token::Integer(1));
            assert_eq!(lexer.next(), Token::Plus);
            assert_eq!(lexer.next(), Token::Minus);
            assert_eq!(lexer.next(), Token::Slash);
            assert_eq!(lexer.next(), Token::Asterisk);
            assert_eq!(lexer.next(), Token::LParen);
            assert_eq!(lexer.next(), Token::RParen);
            assert_eq!(lexer.next(), Token::Assignment);
            assert_eq!(lexer.next(), Token::Not);
            assert_eq!(lexer.next(), Token::Eq);
            assert_eq!(lexer.next(), Token::NotEq);
            assert_eq!(lexer.next(), Token::Lt);
            assert_eq!(lexer.next(), Token::Gt);
            assert_eq!(lexer.next(), Token::LtEq);
            assert_eq!(lexer.next(), Token::GtEq);
            assert_eq!(lexer.next(), Token::SemiColon);
            assert_eq!(lexer.next(), Token::Identifier(String::from("a")));
            assert_eq!(lexer.next(), Token::Identifier(String::from("b")));
            assert_eq!(lexer.next(), Token::Identifier(String::from("foo")));
            assert_eq!(lexer.next(), Token::Identifier(String::from("bar")));
            assert_eq!(lexer.next(), Token::Return);
            assert_eq!(lexer.next(), Token::If);
            assert_eq!(lexer.next(), Token::Else);
            assert_eq!(lexer.next(), Token::While);
            assert_eq!(lexer.next(), Token::For);
            assert_eq!(lexer.next(), Token::Eof);
        }
        {
            let input = String::from("1 + 2");
            let mut lexer = Lexer::new(input);

            assert_eq!(lexer.next(), Token::Integer(1));
            assert_eq!(lexer.next(), Token::Plus);
            assert_eq!(lexer.next(), Token::Integer(2));
            assert_eq!(lexer.next(), Token::Eof);
        }

        {
            let input = String::from("5+20-4");
            let mut lexer = Lexer::new(input);

            assert_eq!(lexer.next(), Token::Integer(5));
            assert_eq!(lexer.next(), Token::Plus);
            assert_eq!(lexer.next(), Token::Integer(20));
            assert_eq!(lexer.next(), Token::Minus);
            assert_eq!(lexer.next(), Token::Integer(4));
            assert_eq!(lexer.next(), Token::Eof);
        }
    }
}
