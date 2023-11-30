use crate::lexer::Token;
use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Instruction {
    Push(i32),
    Duplicate,
    Copy(i32),
    Swap,
    Discard,
    Slide(i32),
    Add,
    Substract,
    Multiply,
    Divide,
    Modulo,
    HeapStore,
    HeapRetrieve,
    MarkLocation(String),
    Call(String),
    Jump(String),
    JumpIfZero(String),
    JumpIfNegative(String),
    EndSubroutine,
    EndProgram,
    OutputChar,
    OutputNumber,
    ReadChar,
    ReadNumber,
}

#[derive(Debug)]
pub struct Parser {
    input: Vec<Token>,
    current: usize,
    pub output: Vec<Instruction>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            input: tokens,
            current: 0,
            output: Vec::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn advance(&mut self) -> &Token {
        self.current += 1;
        &self.input[self.current - 1]
    }

    pub fn parse(&mut self) -> Result<()> {
        while !self.is_at_end() {
            match self.advance() {
                Token::Tab => match self.advance() {
                    Token::Space => self.parse_arithmetic()?,
                    Token::Tab => self.parse_heap_access()?,
                    Token::LineFeed => self.parse_input_output()?,
                },
                Token::Space => self.parse_stack_manipulation()?,
                Token::LineFeed => self.parse_flow_control()?,
            };
        }

        Ok(())
    }

    fn parse_stack_manipulation(&mut self) -> Result<()> {
        let instruction = match self.advance() {
            Token::Space => Instruction::Push(self.parse_number()?),
            Token::Tab => match self.advance() {
                Token::Space => Instruction::Copy(self.parse_number()?),
                Token::LineFeed => Instruction::Slide(self.parse_number()?),
                _ => bail!("invalid stack manipulation instruction"),
            },
            Token::LineFeed => match self.advance() {
                Token::Tab => Instruction::Swap,
                Token::LineFeed => Instruction::Discard,
                Token::Space => Instruction::Duplicate,
            },
        };

        self.output.push(instruction);

        Ok(())
    }

    fn parse_arithmetic(&mut self) -> Result<()> {
        let instruction = match self.advance() {
            Token::Space => match self.advance() {
                Token::Space => Instruction::Add,
                Token::Tab => Instruction::Substract,
                Token::LineFeed => Instruction::Multiply,
            },
            Token::Tab => match self.advance() {
                Token::Space => Instruction::Divide,
                Token::Tab => Instruction::Modulo,
                _ => bail!("invalid arithmetic instruction"),
            },
            _ => bail!("invalid arithmetic instruction"),
        };

        self.output.push(instruction);

        Ok(())
    }

    fn parse_heap_access(&mut self) -> Result<()> {
        let instruction = match self.advance() {
            Token::Space => Instruction::HeapStore,
            Token::Tab => Instruction::HeapRetrieve,
            _ => bail!("invalid heap instruction"),
        };

        self.output.push(instruction);

        Ok(())
    }

    fn parse_flow_control(&mut self) -> Result<()> {
        let instruction = match self.advance() {
            Token::Space => match self.advance() {
                Token::Space => Instruction::MarkLocation(self.parse_label()),
                Token::Tab => Instruction::Call(self.parse_label()),
                Token::LineFeed => Instruction::Jump(self.parse_label()),
            },
            Token::Tab => match self.advance() {
                Token::Space => Instruction::JumpIfZero(self.parse_label()),
                Token::Tab => Instruction::JumpIfNegative(self.parse_label()),
                Token::LineFeed => Instruction::EndSubroutine,
            },
            Token::LineFeed => match self.advance() {
                Token::LineFeed => Instruction::EndProgram,
                _ => bail!("invalid flow control instruction"),
            },
        };

        self.output.push(instruction);

        Ok(())
    }

    fn parse_input_output(&mut self) -> Result<()> {
        let instruction = match self.advance() {
            Token::Space => match self.advance() {
                Token::Space => Instruction::OutputChar,
                Token::Tab => Instruction::OutputNumber,
                _ => bail!("invalid i/o instruction"),
            },
            Token::Tab => match self.advance() {
                Token::Space => Instruction::ReadChar,
                Token::Tab => Instruction::ReadNumber,
                _ => bail!("invalid i/o instruction"),
            },
            _ => bail!("invalid i/o instruction"),
        };

        self.output.push(instruction);

        Ok(())
    }

    fn parse_number(&mut self) -> Result<i32> {
        let sign = match self.advance() {
            Token::Space => 1,
            Token::Tab => -1,
            other => bail!("invalid sign specifier {other:?}"),
        };

        let mut value = 0;

        loop {
            let token = self.advance();
            match token {
                Token::Space => {
                    value <<= 1;
                }
                Token::Tab => {
                    value <<= 1;
                    value += 1;
                }
                Token::LineFeed => break,
            }
        }

        Ok(value * sign)
    }

    fn parse_label(&mut self) -> String {
        let mut label = String::new();

        loop {
            let token = self.advance();
            label.push(match token {
                Token::Space => ' ',
                Token::Tab => '\t',
                Token::LineFeed => break,
            });
        }

        label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_stack_manipulation() {
        let tokens = vec![
            Token::Space,
            Token::Space,
            Token::Tab,
            Token::Tab,
            Token::Tab,
            Token::Space,
            Token::Space,
            Token::Tab,
            Token::Space,
            Token::LineFeed,
        ];

        let mut parser = Parser::new(tokens);
        parser.parse().unwrap();
        let instruction = parser.output.get(0).unwrap();
        assert!(matches!(instruction, Instruction::Push(-50)));
    }

    #[test]
    fn multiple_stack_manipulation() {
        let tokens = vec![
            Token::Space,
            Token::Space,
            Token::Tab,
            Token::Tab,
            Token::Tab,
            Token::Space,
            Token::Space,
            Token::Tab,
            Token::Space,
            Token::LineFeed,
            Token::Space,
            Token::LineFeed,
            Token::Tab,
        ];

        let mut parser = Parser::new(tokens);
        parser.parse().unwrap();
        let first = parser.output.get(0).unwrap();
        let second = parser.output.get(1).unwrap();
        assert!(matches!(first, Instruction::Push(-50)));
        assert!(matches!(second, Instruction::Swap));
    }
}
