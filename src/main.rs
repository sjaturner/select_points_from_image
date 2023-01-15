extern crate clap;
extern crate find_folder;
extern crate piston_window;
use imagesize;

use clap::Parser;
use piston_window::*;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    file: String,

    #[clap(short, long, value_parser, default_value = "1024x768")]
    geometry: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.file);
    let geometry: Vec<u32> = args
        .geometry
        .split("x")
        .map(|v| v.parse().expect("bad geometry argument"))
        .collect();

    let dims = imagesize::size(&path).unwrap();

    let image_width = dims.width as f64;
    let image_height = dims.height as f64;
    let scale_width = geometry[0] as f64 / image_width;
    let scale_height = geometry[1] as f64 / image_height;
    let mut scale = if scale_width < scale_height {
        scale_width
    } else {
        scale_height
    };

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: image", [geometry[0], geometry[1]])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let texture: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &path,
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    window.set_lazy(true);
    let window_width = geometry[0] as f64;
    let window_height = geometry[1] as f64;

    let mut cursor = [0.0, 0.0];
    let mut trans = [-image_width / 2.0, -image_height / 2.0];

    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| {
            cursor = pos;
        });
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                trans[0] += (geometry[0] as f64 / 2.0 - cursor[0]) / scale;
                trans[1] += (geometry[1] as f64 / 2.0 - cursor[1]) / scale;
            }
            if button == Button::Mouse(MouseButton::Right) {
                println!("{} {}", -trans[0], -trans[1]);
            }
        };
        if let Some(mouse_scroll_args) = e.mouse_scroll_args() {
            scale *= if mouse_scroll_args[1] > 0.0 { 1.1 / 1.0 } else { 1.0 / 1.1 };
        };
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);

            image(
                &texture,
                c.transform.trans(window_width / 2.0,window_height / 2.0).scale(scale,scale).trans(trans[0],trans[1]),
                g,
            );

            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, [window_width / 2.0 - 1.0, window_height / 2.0 - 1.0, 2.0, 2.0], c.transform, g);
        });
    }
}
