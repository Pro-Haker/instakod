use std::{
    fs,
    io::{self, BufRead},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

use thiserror::Error;

pub fn parse(path: impl AsRef<Path>) -> Result<Vec<Command>, ParseError> {
    let file = fs::File::open(path.as_ref())?;
    let reader = io::BufReader::new(file);

    let mut commands = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let line = match line.split_once('#') {
            Some((code, _comment)) => code,
            None => &line,
        };
        commands.push(line.trim().parse::<Command>().map_err(|e| {
            <(usize, CommandFromStrErrorKind) as Into<CommandFromStrError>>::into((i + 1, e))
        })?);
    }

    Ok(commands)
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Io error")]
    Io(#[from] io::Error),
    #[error("Failed to parse a command")]
    LineParse(#[from] CommandFromStrError),
}

#[derive(Debug)]
pub enum Command {
    Pvr(Variable),
    Ptx(String),
    Nln,
    Ltv(Variable),
    Set(Variable, VarOrNum),
    Add(Variable, VarOrNum),
    Sub(Variable, VarOrNum),
    Ifj(Condition, JumpDest, JumpDest),
    Jmp(JumpDest),
}

impl FromStr for Command {
    type Err = CommandFromStrErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once(' ');

        let (command_name, rest) = match split {
            Some(s) => s,
            None if s == "NLN" => return Ok(Command::Nln),
            _ => return Err(CommandFromStrErrorKind::CommandName),
        };

        match command_name {
            "PVR" => Ok(Command::Pvr(rest.parse::<Variable>()?)),
            "PTX" => Ok(Command::Ptx(parse_ptx_str(rest)?)),
            "LTV" => Ok(Command::Ltv(rest.parse::<Variable>()?)),
            "SET" => {
                let (variable, var_or_num) = rest
                    .split_once(' ')
                    .ok_or(CommandFromStrErrorKind::SetInvocation)?;
                let variable = variable.parse::<Variable>()?;
                let var_or_num = var_or_num.parse::<VarOrNum>()?;
                Ok(Command::Set(variable, var_or_num))
            }
            "ADD" => {
                let (variable, var_or_num) = rest
                    .split_once(' ')
                    .ok_or(CommandFromStrErrorKind::AddInvocation)?;
                let variable = variable.parse::<Variable>()?;
                let var_or_num = var_or_num.parse::<VarOrNum>()?;
                Ok(Command::Add(variable, var_or_num))
            }
            "SUB" => {
                let (variable, var_or_num) = rest
                    .split_once(' ')
                    .ok_or(CommandFromStrErrorKind::SubInvocation)?;
                let variable = variable.parse::<Variable>()?;
                let var_or_num = var_or_num.parse::<VarOrNum>()?;
                Ok(Command::Sub(variable, var_or_num))
            }
            "IFJ" | "IF" => {
                let mut split = rest.rsplitn(3, ' ');
                let else_ = split
                    .next()
                    .ok_or(CommandFromStrErrorKind::IfjInvocation)?
                    .parse::<JumpDest>()?;
                let then = split
                    .next()
                    .ok_or(CommandFromStrErrorKind::IfjInvocation)?
                    .parse::<JumpDest>()?;
                let condition = split
                    .next()
                    .ok_or(CommandFromStrErrorKind::IfjInvocation)?
                    .parse::<Condition>()?;
                Ok(Command::Ifj(condition, then, else_))
            }
            "JMP" => Ok(Command::Jmp(rest.parse::<JumpDest>()?)),
            _ => Err(CommandFromStrErrorKind::CommandName),
        }
    }
}

#[derive(Debug, Error)]
#[error("Failed to parse a command at line {line}")]
pub struct CommandFromStrError {
    pub line: usize,
    #[source]
    pub source: CommandFromStrErrorKind,
}

impl From<(usize, CommandFromStrErrorKind)> for CommandFromStrError {
    fn from(value: (usize, CommandFromStrErrorKind)) -> Self {
        Self {
            line: value.0,
            source: value.1,
        }
    }
}

#[derive(Debug, Error)]
pub enum CommandFromStrErrorKind {
    #[error("Invalid command name")]
    CommandName,
    #[error("Invalid string")]
    String(#[from] ParsePtxStr),
    #[error("Invalid variable")]
    Variable(#[from] VariableFromStrError),
    #[error("Invalid variable or number")]
    VarOrNum(#[from] VarOrNumFromStrError),
    #[error("Invalid set invocation")]
    SetInvocation,
    #[error("Invalid jump destination")]
    JumpDest(#[from] JumpDestFromStrError),
    #[error("Invalid add invocation")]
    AddInvocation,
    #[error("Invalid sub invocation")]
    SubInvocation,
    #[error("Invalid ifj invocation")]
    IfjInvocation,
    #[error("Invalid condition")]
    Condition(#[from] ConditionFromStrError),
}

#[derive(Debug)]
pub enum Variable {
    A,
    B,
    C,
    D,
}

impl FromStr for Variable {
    type Err = VariableFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            _ => Err(VariableFromStrError::InvalidVariable),
        }
    }
}

#[derive(Debug, Error)]
pub enum VariableFromStrError {
    #[error("Variable has to be: A, B, C or D")]
    InvalidVariable,
}

#[derive(Debug)]
pub enum VarOrNum {
    Variable(Variable),
    Num(u32),
}

impl FromStr for VarOrNum {
    type Err = VarOrNumFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Variable>() {
            Ok(var) => Ok(Self::Variable(var)),
            Err(_) => s
                .parse::<u32>()
                .map(Self::Num)
                .map_err(|_| VarOrNumFromStrError::ParseFailed),
        }
    }
}

#[derive(Debug, Error)]
pub enum VarOrNumFromStrError {
    #[error("Failed to parse as variable or as a number")]
    ParseFailed,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JumpDest {
    Next,
    End,
    Nth(usize),
}

impl FromStr for JumpDest {
    type Err = JumpDestFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NXT" | "NEXT" => Ok(Self::Next),
            "END" => Ok(Self::End),
            num => {
                let num = num.parse::<usize>()?;
                if num == 0 {
                    Err(JumpDestFromStrError::IsZero)
                } else {
                    Ok(JumpDest::Nth(num))
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum JumpDestFromStrError {
    #[error("Invalid jump destination")]
    ParseFailed(#[from] ParseIntError),
    #[error("Jump destination cannot be zero")]
    IsZero,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comp {
    Lt,
    Le,
    Eq,
    Ne,
    Gt,
    Ge,
}

impl FromStr for Comp {
    type Err = CompFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" | "LT" => Ok(Self::Lt),
            "<=" | "LE" => Ok(Self::Le),
            "=" | "EQ" => Ok(Self::Eq),
            "!=" | "NE" => Ok(Self::Ne),
            ">" | "GT" => Ok(Self::Gt),
            ">=" | "GE" => Ok(Self::Ge),
            _ => Err(CompFromStrError::ParseFailed),
        }
    }
}

#[derive(Debug, Error)]
pub enum CompFromStrError {
    #[error("Invalid comparison statement")]
    ParseFailed,
}

#[derive(Debug)]
pub struct Condition {
    pub a: Variable,
    pub comp: Comp,
    pub b: VarOrNum,
}

impl FromStr for Condition {
    type Err = ConditionFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(3, ' ');
        let a = split
            .next()
            .ok_or(ConditionFromStrError::NotEnoughSegments)?
            .parse::<Variable>()?;
        let comp = split
            .next()
            .ok_or(ConditionFromStrError::NotEnoughSegments)?
            .parse::<Comp>()?;
        let b = split
            .next()
            .ok_or(ConditionFromStrError::NotEnoughSegments)?
            .parse::<VarOrNum>()?;

        Ok(Self { a, comp, b })
    }
}

#[derive(Debug, Error)]
pub enum ConditionFromStrError {
    #[error("Not enough segments in condition")]
    NotEnoughSegments,
    #[error("Error while parsing variable in condition")]
    VariableParseError(#[from] VariableFromStrError),
    #[error("Error while parsing comparison in condition")]
    CompParseError(#[from] CompFromStrError),
    #[error("Error while parsing variable or number in condition")]
    VarOrNumParseError(#[from] VarOrNumFromStrError),
}

fn parse_ptx_str(str: &str) -> Result<String, ParsePtxStr> {
    let mut chars = str.trim().chars();
    if chars.next() != Some('"') {
        return Err(ParsePtxStr::NoQuotation);
    }

    let mut result = String::new();
    let mut escaped = false;

    for c in chars {
        if escaped {
            result.push(match c {
                '"' => '"',
                '\\' => '\\',
                other => return Err(ParsePtxStr::InvalidEscape(other)),
            });
            escaped = false;
        } else {
            match c {
                '\\' => escaped = true,
                '"' => return Ok(result),
                c => result.push(c),
            }
        }
    }

    Err(ParsePtxStr::NoQuotation)
}

#[derive(Debug, Error)]
pub enum ParsePtxStr {
    #[error("The string isn't delimitered with the quotation marks")]
    NoQuotation,
    #[error("Invalid character escape sequence: \"\\{0}\"")]
    InvalidEscape(char),
}
