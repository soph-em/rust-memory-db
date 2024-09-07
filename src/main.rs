use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let mut storage = HashMap::<String, Value>::new();

    let stdin = io::stdin();
    for maybe_line in stdin.lock().lines() {
        let line = maybe_line?;
        let splits = line.split(' ');
        let parser = Parser {
            splits: splits.map(|s| s.to_string()).collect(),
        };
        let command = parser.get_command()?;
        // let command = splits
        //     .next()
        //     .ok_or(anyhow!("Expected command, e.g. 'set'"))?;
        match command {
            Command::Set => {
                let key = parser.get_key()?;
                let value = parser.get_value()?;
                // let key = splits.next().ok_or(anyhow!("Expected key"))?;
                // let value = splits.next().ok_or(anyhow!("Expected value"))?;
                storage.insert(key, value);
            }
            Command::Get => {
                let key = parser.get_key()?;
                let value = storage
                    .get(&key)
                    .ok_or(anyhow!("No value found for key: {key}"))?;
                println!("{value}");
            }
        }
    }

    Ok(())
}

struct Parser {
    splits: Vec<String>,
}

impl Parser {
    fn get_command(&self) -> Result<Command> {
        let command = self.splits.get(0).ok_or(anyhow!("Expected command"))?;
        if command == "set" {
            return Result::Ok(Command::Set);
        } else if command == "get" {
            return Result::Ok(Command::Get);
        } else {
            return Err(anyhow!("uh oh"));
        }
    }
    fn get_key(&self) -> Result<String> {
        let key = self.splits.get(1).ok_or(anyhow!("Expected key"))?;
        return Result::Ok(key.to_string());
    }
    fn get_value(&self) -> Result<Value> {
        // let value_length = self.splits.len();
        let value = self.splits.get(2..).ok_or(anyhow!("Expected value"))?;
        return Result::Ok(Value::String(value.join(" ")));
    }
}

enum Command {
    Set,
    Get,
}

enum Value {
    String(String),
    Int(i32),
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(val) => write!(f, "'{}'", val),
            Value::Int(val) => write!(f, "{}", val),
        }
    }
}
