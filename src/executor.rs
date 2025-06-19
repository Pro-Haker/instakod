use std::{
    io::{self, Write},
    num::ParseIntError,
};

use thiserror::Error;

use crate::parser::{Command, Comp, Condition, JumpDest, VarOrNum, Variable};

#[derive(Debug)]
pub struct Executor {
    state: State,
    pc: usize,
    commands_len: usize,
    commands: Vec<Command>,
}

impl Executor {
    pub fn new(commands: Vec<Command>) -> Self {
        Self {
            state: Default::default(),
            commands_len: commands.len(),
            pc: 0,
            commands,
        }
    }

    pub fn execute(mut self) -> Result<(), ExecutingError> {
        let commands = self.commands;
        self.commands = Vec::new();

        while self.pc < self.commands_len {
            self.pc += 1;
            match &commands[self.pc - 1] {
                Command::Pvr(var) => print!("{}", self.state.get(var)),
                Command::Ptx(text) => print!("{text}"),
                Command::Nln => println!(),
                Command::Ltv(variable) => {
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim().parse::<u64>()? as i64;
                    *self.state.get_mut(variable) = input;
                }
                Command::Set(variable, var_or_num) => {
                    let num = self.state.get_var_or_num(var_or_num);
                    *self.state.get_mut(variable) = num;
                }
                Command::Add(variable, var_or_num) => {
                    let num = self.state.get_var_or_num(var_or_num);
                    *self.state.get_mut(variable) += num;
                }
                Command::Sub(variable, var_or_num) => {
                    let num = self.state.get_var_or_num(var_or_num);
                    *self.state.get_mut(variable) -= num;
                }
                Command::Ifj(condition, jump_dest, jump_dest1) => {
                    if self.state.evaluate_condition(condition) {
                        self.jump(jump_dest);
                    } else {
                        self.jump(jump_dest1);
                    }
                }
                Command::Jmp(jump_dest) => self.jump(jump_dest),
            }
        }

        Ok(())
    }

    fn jump(&mut self, jump_dest: &JumpDest) {
        match jump_dest {
            JumpDest::Next => {}
            JumpDest::End => self.pc = self.commands_len,
            JumpDest::Nth(num) => self.pc = *num - 1,
        }
    }
}

#[derive(Debug, Error)]
pub enum ExecutingError {
    #[error("Io error")]
    Io(#[from] io::Error),
    #[error("Failed to parse a number")]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug, Default)]
pub struct State {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl State {
    fn get(&self, var: &Variable) -> i64 {
        match var {
            Variable::A => self.a,
            Variable::B => self.b,
            Variable::C => self.c,
            Variable::D => self.d,
        }
    }

    fn get_mut(&mut self, var: &Variable) -> &mut i64 {
        match var {
            Variable::A => &mut self.a,
            Variable::B => &mut self.b,
            Variable::C => &mut self.c,
            Variable::D => &mut self.d,
        }
    }

    fn get_var_or_num(&self, var_or_num: &VarOrNum) -> i64 {
        match var_or_num {
            VarOrNum::Variable(variable) => match variable {
                Variable::A => self.a,
                Variable::B => self.b,
                Variable::C => self.c,
                Variable::D => self.d,
            },
            VarOrNum::Num(num) => *num as i64,
        }
    }

    fn evaluate_condition(&self, condition: &Condition) -> bool {
        let a = self.get(&condition.a);
        let b = self.get_var_or_num(&condition.b);

        match condition.comp {
            Comp::Lt => a < b,
            Comp::Le => a <= b,
            Comp::Eq => a == b,
            Comp::Ne => a != b,
            Comp::Gt => a > b,
            Comp::Ge => a >= b,
        }
    }
}
