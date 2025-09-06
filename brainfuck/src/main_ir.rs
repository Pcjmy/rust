mod opcode;

use std::io::prelude::*;
use opcode::Opcode;

#[derive(Debug, PartialEq)]
pub enum IR {
  SHR(u32),
  SHL(u32),
  ADD(u32),
  SUB(u32),
  PUTCHAR,
  GETCHAR,
  JIZ(u32),
  JNZ(u32),
}

pub struct Code {
  pub instrs: Vec<IR>,
}

impl Code {
  pub fn from(data: Vec<Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
    let mut instrs: Vec<IR> = Vec::new();
    let mut jstack: Vec<u32> = Vec::new();
    for e in data {
      match e {
        Opcode::SHR => {
          match instrs.last_mut() {
            Some(IR::SHR(x)) => {
              *x += 1;
            }
            _ => {
              instrs.push(IR::SHR(1));
            }
          }
        },
        Opcode::SHL => {
          match instrs.last_mut() {
            Some(IR::SHL(x)) => {
              *x += 1;
            }
            _ => {
              instrs.push(IR::SHL(1));
            }
          }
        },
        Opcode::ADD => {
          match instrs.last_mut() {
            Some(IR::ADD(x)) => {
              let (b, _) = x.overflowing_add(1);
              *x = b;
            }
            _ => {
              instrs.push(IR::ADD(1));
            }
          }
        },
        Opcode::SUB => {
          match instrs.last_mut() {
            Some(IR::SUB(x)) => {
              let (b, _) = x.overflowing_add(1);
              *x = b;
            }
            _ => {
              instrs.push(IR::SUB(1));
            }
          }
        },
        Opcode::PUTCHAR => {
          instrs.push(IR::PUTCHAR);
        },
        Opcode::GETCHAR => {
          instrs.push(IR::GETCHAR);
        },
        Opcode::LB => {
          instrs.push(IR::JIZ(0));
          jstack.push((instrs.len() - 1) as u32);
        },
        Opcode::RB => {
          let j = jstack.pop().ok_or("pop from empty list")?;
          instrs.push(IR::JIZ(j));
          let instrs_len = instrs.len();
          match &mut instrs[j as usize] {
            IR::JIZ(x) => {
              *x = (instrs_len - 1) as u32;
            }
            _ => unreachable!()
          }
        }
      }
    }
    Ok(Code {instrs})
  }
}

struct Interpreter {
  stack: Vec<u8>,
}

impl Interpreter {
  fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let opcode_code = opcode::Code::from(data)?;
    let code = Code::from(opcode_code.instrs)?;
    let code_len = code.instrs.len();
    let mut pc:usize = 0;
    let mut ps:usize = 0;

    loop {
      if pc >= code_len {
        break;
      }

      match code.instrs[pc] {
        IR::SHR(x) => {
          ps += x as usize;
          if ps >= self.stack.len() {
            let expand = ps - self.stack.len() + 1;
            for _ in 0..expand {
              self.stack.push(0);
            }
          }
        },
        IR::SHL(x) => {
          for i in 0..x {
            if ps != 0 {
              ps -= 1;
            } else {
              break;
            }
          }
        },
        IR::ADD(x) => {
          self.stack[ps] = self.stack[ps].overflowing_add(x as u8).0;
        },
        IR::SUB(x) => {
          self.stack[ps] = self.stack[ps].overflowing_sub(x as u8).0;
        },
        IR::PUTCHAR => {
          std::io::stdout().write_all(&[self.stack[ps]])?;
        },
        IR::GETCHAR => {
          let mut buf: Vec<u8> = vec![0; 1];
          std::io::stdin().read(&mut buf)?;
          self.stack[ps] = buf[0];
        },
        IR::JIZ(x) => {
          if self.stack[ps] == 0x00 {
            pc = x as usize;
          }
        },
        IR::JNZ(x) => {
          if self.stack[ps] != 0x00 {
            pc = x as usize;
          }
        }
      }
      pc += 1;
    }

    Ok(())
  }
}


fn main () {}
