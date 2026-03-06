use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
enum Cli {
    List,
    Add {
        name: String,
        age: u64,
        email: String,
    },
}
#[derive(Serialize, Deserialize)]
struct UserData {
    name: String,
    age: u64,
    email: String,
}

impl UserData {
    fn save_all(users: &Vec<Self>) -> Result<()> {
        let json = serde_json::to_string_pretty(users).context("JsonError")?;
        std::fs::write("user.json", json).context("ErrorFile")?;
        Ok(())
    }

    fn load_all() -> Result<Vec<Self>> {
        let content = fs::read_to_string("user.json").context("Error not file")?;
        let users: Vec<Self> = serde_json::from_str(&content).context("Errorserde")?;
        Ok(users)
    }
}
fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args {
        Cli::List => {
            let users = UserData::load_all().unwrap_or_default();
            println!("--------All names-------------");
            for (i, u) in users.iter().enumerate() {
                println!("{}.{}|{}|{}", i + 1, u.name, u.age, u.email);
            }
        }
        Cli::Add { name, age, email } => {
            let mut users = UserData::load_all().unwrap_or_default();
            let new_user = UserData { name, age, email };
            users.push(new_user);
            UserData::save_all(&users)?;
            println!("Flnish");
        }
    }
    Ok(())
}
