use std::{fs, path::Path};

use env_logger::Env;
use game::GameConfig;
use image::GenericImageView;
use log::{debug, error, info, set_max_level, trace, warn};
use maps::{World, WorldManifest};
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

    let maps_names:Vec<String> = fs::read_dir("./worlds")
    .expect("Couldnt Get list of worlds") // Error out if we didnt get a list of documents
    .map(|path| {
        let name = path.expect("Couldnt unwrap path, try again I guess?") //Pretty Useless error but im alergic to unwrap
        .path().file_name().unwrap().to_str().unwrap().to_string(); // Map every path into the name of it into a string
        trace!("Located Map {}",name);
        name
    }).collect(); // Turn it into a vector(Not the math type AHAHAH)

    info!("Found {} Map(s)!",maps_names.len());

    //Now we should map this into memory and load the images

    let maps:Vec<World> = maps_names.into_iter().map(|name| {
        let full_path = format!("./worlds/{}",name);
        trace!("Attemping to load : {}",full_path);

        let manifest_path = format!("{}/manifest.toml",full_path);
        if !Path::new(&manifest_path).exists() {
            panic!("Coulnt Find Manifest for world {}",name)
        }

        let game_config:WorldManifest = toml::from_str( // load the manifest file, it contains all infromation about the world
            &fs::read_to_string(manifest_path) // Read file
            .expect("File not found, but we found it earlier, Hmmmmm")) // Error if we couldnt read it
            .expect("Couldnt Parse the document, make sure the toml file is properly formated"); // Error if it wasent properly formated
        trace!("Loaded manifest!");

        //Lets get the depth image
        let depth_img = image::open(format!("{}/{}",full_path,game_config.maps.depth)).expect("Couldnt Load Depth Image"); // Load the image and hope it world
        if depth_img.dimensions() != (1024,1024){warn!("Images not size of 1024,1024 are not fully supported, expect issues bbg")} //Maps might be wonky if not 1024x1024
        let depth_img = depth_img.to_luma8(); // The DepthBuffer Doesnot need color, so make it grayscale
        let mut depth_buffer:Box<[u8; 1024*1024]>= Box::new([0; 1024*1024]);
        for i in 0..(1024*1024){
            let index:u32 = i.try_into().unwrap();
            depth_buffer[i] = depth_img.get_pixel(index % 1024 , index / 1024).0[0];
        }
        trace!("Loaded Current Depth Map!");

        //Ok now its timmefor COLOR
        let colour_img = image::open(format!("{}/{}",full_path,game_config.maps.colour)).expect("Couldnt Load Colour Image"); // Load the image and hope it world
        if colour_img.dimensions() != (1024,1024){warn!("Images not size of 1024,1024 are not fully supported, expect issues bbg")} //Maps might be wonky if not 1024x1024
        let colour_img = colour_img.to_rgb8(); // The DepthBuffer Doesnot need color, so make it grayscale
        let mut colour_buffer:Box<[(u8,u8,u8); 1024*1024]>= Box::new([(0,0,0); 1024*1024]);
        for i in 0..(1024*1024){
            let index:u32 = i.try_into().unwrap();
            let pixel = colour_img.get_pixel(index % 1024 , index / 1024).0;
            colour_buffer[i] = (pixel[0],pixel[1],pixel[2])
        }
        trace!("Loaded Current Current Map!");

        World{
            manifest:   game_config,
            colour_map: colour_buffer,
            depth_map:  depth_buffer,
        }
    }).collect();

    info!("Loaded {} Map(s)!",maps.len());

    //Now we find the innitial map so we can call something in render()

    let initial_map = maps.iter().position(|m| {
        m.manifest.name == game_config.initial_map
    }).expect("Initial Map Was Not found, is the name correct?");

    let initial_map = &maps[initial_map];
    info!("Loaded initial Map {}",initial_map.manifest.name);


    //Now lets setup Rendering Core :3
    


}
