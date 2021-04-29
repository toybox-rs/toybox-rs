/**
    Steps to creating a new game.
    - get a name, make sure it doesn't clash
    - modify top-level Cargo.toml to include the new package
    - create package
    - modify toybox/Cargo.toml
    - re-generate toybox/src/lib.rs
*/
use std::{convert::TryInto, env, error::Error, io::Write};
use std::fs::{read_to_string, create_dir_all};
use std::fs::File;
use toml;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
struct Games {
    games : Vec<(String, String, String)>
}
#[derive(Deserialize, Clone, Serialize)]
struct TBCargo {
    workspace : Workspace
}

#[derive(Deserialize, Clone, Serialize)]
struct Workspace {
    members: Vec<String>
}

fn add_to_games(game: String, mainclass: String) -> Result<(), String> {
    // Get the existing game list
    let games : Games = toml::from_str(&read_to_string("Games.toml").unwrap()).unwrap();
    let mut new_games = games.clone();
    
    // See if there are any clashes
    for (old_game, _, _) in games.games {
        if old_game.eq(&game) {
            return Err(format!("{} already exists", game))
        }
    }

    // Add the new game to the game list
    new_games.games.push((game.to_string(), game.to_string(), mainclass.to_string()));
    let new_games_toml = toml::to_string(&new_games).unwrap();
    let mut new_games_file = File::create("Games.toml").unwrap();
    match new_games_file.write_all(new_games_toml.as_bytes()) {
        Err(msg) => Err(msg.to_string()),
        _ => Ok(())
    }
}

fn add_to_workspace(dir: String) -> Result<(), String> {
    // Get the existing Cargo.toml
    let cargo_toml : TBCargo = toml::from_str(&read_to_string("Cargo.toml").unwrap()).unwrap();
    let mut new_workspace = cargo_toml.clone();
    new_workspace.workspace.members.push(dir);
    let new_cargo_toml = toml::to_string(&new_workspace).unwrap();
    let mut cargo_file = File::create("Cargo.toml").unwrap();
    match cargo_file.write_all(new_cargo_toml.as_bytes()) {
        Err(msg) => Err(msg.to_string()),
        _ => Ok(())
    }
}

fn create_project_files(game: String, dir: String) -> Result<(), std::io::Error>{
    create_dir_all([dir.clone(), "Cargo.toml".to_string()].join(&std::path::MAIN_SEPARATOR.to_string()))?;
    create_dir_all([dir.clone(), "src".to_string(), "lib.rs".to_string()].join(&std::path::MAIN_SEPARATOR.to_string()))?;
    create_dir_all([dir.clone(), "src".to_string(), "types.rs".to_string()].join(&std::path::MAIN_SEPARATOR.to_string()))?;
    create_dir_all([dir.clone(), "src".to_string(), "resources".to_string()].join(&std::path::MAIN_SEPARATOR.to_string()))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Do all I/O here

    // First get the game/directory/package name
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err("Forgot the game name argument!".to_string())?;
    }
    let arg = &args[1];
    let dir = if arg.starts_with("tb_") { "".to_string() } else { "tb_".to_string() } + &arg;
    let game = if arg.starts_with("tb_") { arg.get(3..).unwrap().to_string() } else { arg.clone() };
    // Capitalize the first letter to make the main class
    let mut v : Vec<char> = game.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    let mainclass : String = v.into_iter().collect();


    // Let the processing begin!
    // add_to_games(game.clone(), mainclass.clone())?;
    // add_to_workspace(dir.clone())?;
    create_project_files(game.clone(), dir.clone())?;

    println!("Successfully created {}!", arg);
    Ok(())
}
