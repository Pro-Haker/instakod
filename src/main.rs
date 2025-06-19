use std::env;

use anyhow::{Context, Result as AnyResult};

use crate::{executor::Executor, parser::parse};

mod executor;
mod parser;

fn main() -> AnyResult<()> {
    let filename = env::args().nth(1).context("No file name given")?;
    Executor::new(parse(filename)?).execute()?;
    Ok(())
}
