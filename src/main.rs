use std::{fs, path::Path};

use env_logger::Env;
use game::GameConfig;
use log::{debug, error, info, set_max_level, trace, warn};
use maps::WorldManifest;
mod maps;
mod game;

fn main() {

    colog::init(); // Setup Logging
    log_panics::init(); //Setup Logging library to unwind and not stderr
    info!("Welcome to Float Engine"); // Hello Message

    if !Path::new("game.toml").exists() {
        panic!("Couldnt find ./worlds, Make sure the directory exsits")
    }else {
        trace!("Found game.toml")
    }

    if !Path::new("./worlds").exists() {
        panic!("Couldnt find ./worlds, Make sure the directory exsits")
    }else {
        trace!("Found worlds/")
    }

    let game_config:GameConfig = toml::from_str(
        &fs::read_to_string("./game.toml") // Read file
        .expect("File not found, but we found it earlier, Hmmmmm")) // Error if we couldnt read it
        .expect("Couldnt Parse the document, make sure the toml file is properly formated"); // Error if it wasent properly formated
    trace!("Loaded Game Config!");


    //Now we have the game config imported, we should see what maps we have avaliable to us :3

    let maps:Vec<String> = fs::read_dir("./worlds")
    .expect("Couldnt Get list of documents") // Error out if we didnt get a list of documents
    .map(|path| {
        let name = path.expect("Couldnt unwrap path, try again I guess?") //Pretty Useless error but im alergic to unwrap
        .path().file_name().unwrap().to_str().unwrap().to_string(); // Map every path into the name of it into a string
        trace!("Loaded Map {}",name);
        name
    }).collect(); // Turn it into a vector(Not the math type AHAHAH)

    info!("Found {} Map(s)!",maps.len());

    //Now we should map this into memory and load the images

    let maps:Vec<_> = maps.into_iter().map(|name| {
        let full_path = format!("./worlds/{}",name);
        trace!("Attemping to load : {}",full_path);

        let manifest_path = format!("{}/manifest.toml",full_path);
        if !Path::new(&manifest_path).exists() {
            panic!("Coulnt Find Manifest for world {}",name)
        }

        let game_config:WorldManifest = toml::from_str(
            &fs::read_to_string(manifest_path) // Read file
            .expect("File not found, but we found it earlier, Hmmmmm")) // Error if we couldnt read it
            .expect("Couldnt Parse the document, make sure the toml file is properly formated"); // Error if it wasent properly formated
        trace!("Loaded manifest!");

        full_path
    }).collect();




}
