#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Space,
    Tab,
    LineFeed,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }

    pub fn lex(&self) -> Vec<Token> {
        self.input
            .chars()
            .filter_map(|chr| match chr {
                ' ' => Some(Token::Space),
                '\t' => Some(Token::Tab),
                '\n' => Some(Token::LineFeed),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let lexer = Lexer::new("aa \n  comment \t\n\t");
        let tokens = lexer.lex();

        assert_eq!(
            tokens,
            vec![
                Token::Space,
                Token::LineFeed,
                Token::Space,
                Token::Space,
                Token::Space,
                Token::Tab,
                Token::LineFeed,
                Token::Tab
            ]
        );
    }
}
