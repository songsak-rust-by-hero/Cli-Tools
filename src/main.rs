use clap::Parser;
use serde::{Serialize,Deserialize};
use std::fs;
use anyhow::{Result,Context};

#[derive(Parser)]
struct Cli{
    name: String,
    age: u64,
    email: String,
}
#[derive(Serialize,Deserialize)]
struct UserData{
    name:String,
    age:u64,
    email:String
}
impl From<Cli> for UserData {
    fn from (args:Cli)-> Self {
        Self { name: args.name,
        age:args.age,
        email:args.email,
        }
    }
}
impl UserData{
    fn save(&self)-> Result<()>{
        let json = serde_json::to_string_pretty(self).context("JsonError")?;
        std::fs::write("user.json",json).context("ErrorFile")?;
        Ok(())
    }

    fn load()-> Result<Self>{
        let content = fs::read_to_string("user.json").context("Error not file")?;
        let user: Self =
            serde_json::from_str(&content).context("Errorserde")?;
        Ok(user)
}
}
fn main()->anyhow::Result<()> {
    match UserData::load() {
        Ok(user) => { println!("Fond :{}(Age{}):(email) {}",user.name,user.age,user.email);
        }
        Err(_) =>{ println!("No existing user file found, proceeding to create a new one.");
    }
    }
   let args = Cli::parse();
   let user = UserData::from(args);
   user.save().context("Error")?;
   println!("finish");
   Ok(())
}
