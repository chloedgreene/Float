use macroquad::{prelude::*};
use std::f32::consts::PI;

use image::{DynamicImage, GenericImageView};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[macro_use]
extern crate lazy_static;

const RENDER_DETAIL: f32 = 1024.;

const TO_RADIANS: f32 = PI / 180.0;

const HEIGHT_SCALE: i32 = 15;

mod titlescreen;

fn window_conf() -> Conf {
    Conf {
        window_title: "Float".to_owned(),
        fullscreen: false,
        window_width: 970,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //lil basic loading screen
    let (sky_colour, cmap, dmap) = titlescreen::load_code().await;

    let mut camx: f32 = 0.;
    let mut camy: f32 = 4000.;
    let mut camz: f32 = 0.;
    let mut roty: f32 = 20.;
    let mut rotp: f32 = 0.;
    let mut rotr: f32 = 0.;
    let mut fov = 50.;
    let mut mov_speed: f32 = 2.;
    let rot_speed: f32 = 5.;

    loop {
        //I NOW THIS SHOULD BE A SWITCH STATEMENT OR A PATTERN MATCH
        //SHUT UPPPPP I DONT CARE, IT MESSY BUT WORKS GOOD ENOUGH

        if is_key_down(KeyCode::Tab){
            mov_speed = 7.;
        }else {
            mov_speed = 2.;
        }

        //Moving code
        //I HATE TRIG I ALWAYS MESSUP SIN AND COS LIKE OMFGGGGGGG
        if is_key_down(KeyCode::W) {
            camx += (roty * TO_RADIANS).cos() * mov_speed;
            camz += (roty * TO_RADIANS).sin() * mov_speed;
        }
        if is_key_down(KeyCode::S) {
            camx -= (roty * TO_RADIANS).cos() * mov_speed;
            camz -= (roty * TO_RADIANS).sin() * mov_speed;
        }

        if is_key_down(KeyCode::D) {
            roty -= rot_speed;
        }
        if is_key_down(KeyCode::A) {
            roty += rot_speed;
        }

        if is_key_down(KeyCode::E) {
            rotr -= rot_speed;
        }
        if is_key_down(KeyCode::Q) {
            rotr += rot_speed;
        }

        //Pitch up of down camera
        if is_key_down(KeyCode::Up) {
            rotp -= rot_speed * 2.;
        }
        if is_key_down(KeyCode::Down) {
            rotp += rot_speed * 2.;
        }

        //Setup Fov
        if is_key_down(KeyCode::U) {
            fov += 1.;
        }
        if is_key_down(KeyCode::J) {
            fov -= 1.;
        }


        //Move Y Pos with ctrl and shift
        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            camy -= mov_speed;
        }
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            camy += mov_speed;
        }

        clear_background(sky_colour);

        draw(
            800.,
            screen_height() / 12.,
            100. - rotp,
            fov    * TO_RADIANS,
            roty  * TO_RADIANS,
            rotr  * TO_RADIANS,
            camx,
            camy,
            camz,
            &cmap,
            &dmap,
        );

        draw_text(
            format!("pos:\nx:{}\ny:{}\nz:{}", camx, camy, camz).as_str(),
            5.,
            18.,
            24.,
            WHITE,
        );

        draw_text(
            format!("fps: {}",get_fps()).as_str(),
            5.,
            38.,
            24.,
            WHITE,
        );

        next_frame().await;
    }
}

//     p, phi, height, horizon, scale_height, distance, screen_width, screen_height
// {0,0), 0  , 50    , 120    , 120         ,      300,          800,           600 )

#[inline]
pub fn draw(
    view_distance: f32,
    height_scale: f32,
    horizon: f32,
    fov: f32,
    roty: f32,
    _rotr: f32,
    camx: f32,
    camy: f32,
    camz: f32,
    cmap: &DynamicImage,
    dmap: &DynamicImage,
) {
    let (width, height) = (
        macroquad::window::screen_width(),
        macroquad::window::screen_height(),
    );
    // visibility array
    let mut visibility = vec![height; width as usize];
    let mut z = 1.0;

    let (_,_,mapsizex,mapsizey) = dmap.bounds();


    while z < view_distance {
        // projection
        let invz = 1.0 / z * height_scale;

        // find line on map. this corresponds to 90 degree FOV
        let left = roty + fov  * 0.5;
        let right = roty - fov * 0.5;

        // draw left to right
        let r_delta = (right - left) / width;
        for (screen_x, visible_y) in visibility.iter_mut().enumerate() {
            // get position on map

            let phi = left + r_delta * screen_x as f32;
            let map_x = camx + phi.cos() * z;
            let map_y = camz + phi.sin() * z;
            println!("{}",phi);


            // get color and height from map at the point

            let mut color = [0, 0, 0, 255];
            let mut map_height = 0;

            if map_x < mapsizex as f32 && map_y < mapsizey as f32 && map_x > 1. && map_y > 1. {
                color = cmap.get_pixel(map_x as u32, map_y as u32).0;
                map_height = dmap.get_pixel(map_x as u32, map_y as u32).0[1] as i32;
                map_height *= HEIGHT_SCALE;
            }

            // perspective projection for height
            // aka i dont understand this, i just took the math from the paper :D
            let mut y = camy - map_height as f32;
            y = (y * invz + horizon).clamp(0.0, height);

            if y < *visible_y {
                draw_vertical_line(screen_x as f32 + 1., y, *visible_y,_rotr, color);
                *visible_y = y
            }
        }

        z += (z / RENDER_DETAIL).max(1.0);
    }
}

//save overhead of function and returning
#[inline]
fn draw_vertical_line(x: f32, y: f32, height: f32,_rotr: f32, color: [u8; 4]) {
    let g = Color::new(
        color[0] as f32 / 255.,
        color[1] as f32 / 255.,
        color[2] as f32 / 255.,
        color[3] as f32,
    );

    draw_line(x, y, x, height, 1., g)
}
