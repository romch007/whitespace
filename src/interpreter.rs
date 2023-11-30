use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Result};

use crate::parser::Instruction;

#[derive(Debug)]
pub struct VM {
    instruction_ptr: usize,
    pub stack: Vec<i32>,
    labels: HashMap<String, usize>,
    pub heap: Vec<i32>,
}

impl VM {
    pub fn new() -> Self {
        Self::with_heap_size(1024)
    }

    pub fn with_heap_size(heap_size: usize) -> Self {
        Self {
            instruction_ptr: 0,
            stack: Vec::new(),
            labels: HashMap::new(),
            heap: vec![0; heap_size],
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) -> Result<()> {
        for (i, instr) in instructions.iter().enumerate() {
            if let Instruction::MarkLocation(label) = instr {
                self.labels.insert(label.clone(), i);
            }
        }

        loop {
            let stack_len = self.stack.len();

            let instruction = instructions
                .get(self.instruction_ptr)
                .ok_or_else(|| anyhow!("no more instructions"))?;

            match instruction {
                Instruction::Push(number) => {
                    self.stack.push(*number);
                }
                Instruction::Duplicate => {
                    let element = self.peek_stack()?;

                    self.stack.push(*element);
                }
                Instruction::Copy(_) => unimplemented!("copy"),
                Instruction::Swap => {
                    self.stack.swap(stack_len - 1, stack_len - 2);
                }
                Instruction::Discard => {
                    self.pop_stack()?;
                }
                Instruction::Slide(_) => unimplemented!("slide"),
                Instruction::Add => {
                    let left = self.pop_stack()?;
                    let right = self.pop_stack()?;

                    self.stack.push(left + right);
                }
                Instruction::Substract => {
                    let left = self.pop_stack()?;
                    let right = self.pop_stack()?;

                    self.stack.push(left - right);
                }
                Instruction::Multiply => {
                    let left = self.pop_stack()?;
                    let right = self.pop_stack()?;

                    self.stack.push(left * right);
                }
                Instruction::Divide => {
                    let left = self.pop_stack()?;
                    let right = self.pop_stack()?;

                    self.stack.push(
                        left.checked_div(right)
                            .ok_or_else(|| anyhow!("trying to divide {left} by zero"))?,
                    );
                }
                Instruction::Modulo => {
                    let left = self.pop_stack()?;
                    let right = self.pop_stack()?;
                    self.stack.push(left % right);
                }
                Instruction::HeapStore => {
                    let value = self.pop_stack()?;
                    let address = self.pop_stack()?;

                    self.store_heap(address, value)?;
                }
                Instruction::HeapRetrieve => {
                    let address = self.pop_stack()?;

                    let value = self.get_heap(address)?;

                    self.stack.push(value);
                }
                Instruction::MarkLocation(_) => {}
                Instruction::Call(label) => {
                    self.stack.push(i32::try_from(self.instruction_ptr)? + 1);
                    self.jump(label)?;
                }
                Instruction::Jump(label) => {
                    self.jump(label)?;
                }
                Instruction::JumpIfZero(label) => {
                    let top = self.peek_stack()?;

                    if *top == 0 {
                        self.jump(label)?;
                    }
                }
                Instruction::JumpIfNegative(label) => {
                    let top = self.peek_stack()?;

                    if *top < 0 {
                        self.jump(label)?;
                    }
                }
                Instruction::EndSubroutine => {
                    let addr = self.pop_stack()?;
                    self.instruction_ptr = usize::try_from(addr).with_context(|| "invalid addr")?;
                }
                Instruction::EndProgram => break Ok(()),
                Instruction::OutputChar => {
                    let element = self.pop_stack()?;

                    let chr = char::from_u32(
                        u32::try_from(element).with_context(|| "invalid character in stack")?,
                    )
                    .ok_or_else(|| anyhow!("invalid character"))?;

                    print!("{chr}");
                }
                Instruction::OutputNumber => {
                    let element = self.pop_stack()?;
                    print!("{element}");
                }
                Instruction::ReadChar => {
                    let chr = console::Term::stdout()
                        .read_char()
                        .with_context(|| "reading a character")?;

                    self.stack.push(chr as i32);
                }
                Instruction::ReadNumber => {
                    let mut line = String::new();

                    std::io::stdin()
                        .read_line(&mut line)
                        .with_context(|| "reading line")?;

                    self.stack.push(
                        line.trim()
                            .parse()
                            .with_context(|| "parsing line to number")?,
                    );
                }
            };

            self.instruction_ptr += 1;
        }
    }

    fn pop_stack(&mut self) -> Result<i32> {
        self.stack
            .pop()
            .ok_or_else(|| anyhow!("empty stack during pop"))
    }

    fn peek_stack(&self) -> Result<&i32> {
        self.stack
            .last()
            .ok_or_else(|| anyhow!("empty stack during peek"))
    }

    fn jump(&mut self, label: &String) -> Result<()> {
        self.instruction_ptr = *self
            .labels
            .get(label)
            .ok_or_else(|| anyhow!("label not found"))?;

        Ok(())
    }

    fn get_heap(&self, address: i32) -> Result<i32> {
        let address = usize::try_from(address).with_context(|| "invalid address")?;

        if address >= self.heap.len() {
            bail!("heap overflow");
        }

        Ok(self.heap[address])
    }

    fn store_heap(&mut self, address: i32, value: i32) -> Result<()> {
        let address = usize::try_from(address).with_context(|| "invalid address")?;

        if address >= self.heap.len() {
            bail!("heap overflow");
        }

        self.heap[address] = value;

        Ok(())
    }
}
