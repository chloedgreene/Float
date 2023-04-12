
use std::{fs::{self, File}, path, io::Read};

use hex_color::HexColor;
use image::DynamicImage;
use macroquad::prelude::*;
use serde_json::Value;

use image::{io::Reader as ImageReader, GenericImageView, Pixel};


pub async fn load_code() -> (Color,DynamicImage,DynamicImage){

    let paths = fs::read_dir("./worlds").unwrap();
    let count = paths.count();

    let mut selection:f32 = 0.;
    let mut current:f32 = 0.;
    let mut final_selection:f32 = 1024.;

    loop {
        
        clear_background(BLACK);

        draw_text("float", 25., 35., 45., WHITE);
        draw_text("load_map", 130., 35., 25., WHITE);

        let paths = fs::read_dir("./worlds").unwrap();
        
        current = 0.;
        for path in paths {

            
            let mut g = ">  ";
            if current as usize == (selection) as usize{
                g = "»» ";
            }

            if final_selection as usize == current as usize{

                //Load data from world file

                g = "Loading...";
                let path_raw = path.unwrap().path().display().to_string();


                let path_json = format!("{}/game.json",path_raw);

                let mut file = File::open(path_json).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                println!("{}",contents);
                
                let v: Value = serde_json::from_str(&contents).unwrap();
                let sky_colour = HexColor::parse(v["sky_colour"].as_str().unwrap()).unwrap();
                let colour_map = format!( "{}/{}" , path_raw,v["maps"]["colour_map"].as_str().unwrap() );
                let  depth_map = format!( "{}/{}" ,  path_raw,v["maps"]["depth_map"].as_str().unwrap() );

                println!("Sky Colour: {}\nColour Map: {}\nDepth: {}",sky_colour,colour_map,depth_map);

                let cmap = ImageReader::open(colour_map).unwrap()
                .with_guessed_format()
                .expect("Cant find format")
                .decode()
                .expect("Cant load image");
                
                let dmap = ImageReader::open(depth_map).unwrap()
                .with_guessed_format()
                .expect("Cant find format")
                .decode()
                .expect("Cant load image");

                let sky_colour = Color::from_rgba(sky_colour.r, sky_colour.g, sky_colour.b, sky_colour.a);

                return (sky_colour,cmap,dmap);

            }
            

            draw_text(format!("{}{}",g,path.unwrap().path().display()).as_str(), 25., 85. + (25. * current), 25., WHITE);
            current = current + 1.;
        }

        println!("{}",current);

        if is_key_down(KeyCode::Up){
            selection += 0.25;
            if selection as usize > count-1{
                selection = 0.;
            }
        }
        if is_key_down(KeyCode::Down){
            selection += 0.25;
            if selection as usize > count-1{
                selection = 0.;
            }
        }
        if is_key_down(KeyCode::Enter){
            final_selection = selection;
        }

        
        next_frame().await;
    }

}