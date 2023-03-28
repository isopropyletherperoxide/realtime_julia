use std::str::FromStr;

use color_processing::Color;
use image::{Rgb, RgbImage};
use nannou::prelude::*;
use num::complex::Complex;

const HEIGHT: u32 = 256;
const WIDTH: u32 = 256;

#[derive(Debug, Clone)]
enum FractalType {
    Julia,
    Mandelbrot,
}

struct Model {
    texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).update(update).run();
}


#[derive(Debug, Clone)]
struct Config {
    height: u32,
    width: u32,
    scale_fac: f64,
    fractal_type: FractalType,
    julia_r: f64,
    julia_i: f64,
    contrast: u8,
    colors: bool,
    colors_saturation: f64,
    colors_value: f64,
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("framebuf.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    model.texture = texture;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    //    let r = nannou::rand::random_range(0.1, 1.0);
    let r = app.mouse.x;
    // println!("{r}");
    let r = map_range(r, -(WIDTH as f32 / 2.0), WIDTH as f32 / 2.0, -2.0, 2.0);
    println!("{r}");
    let i = app.mouse.y;
    //  println!("{i}");
    let i = map_range(i, -(HEIGHT as f32 / 2.0), HEIGHT as f32 / 2.0, -2.0, 2.0);
    println!("{i}");
    let frac_gen_config = Config {
        height: HEIGHT,
        width: WIDTH,
        scale_fac: 1.0,
        fractal_type: FractalType::Julia,
        julia_r: r,
        julia_i: i,
        contrast: 1,
        colors: true,
        colors_saturation: 1.0,
        colors_value: 0.6,
    };
    let img_buf = RgbImage::new(HEIGHT, WIDTH);
    let img_buf = fill(img_buf, frac_gen_config);
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("nature_1.jpg");
    img_buf.save(&img_path).unwrap();

    draw.texture(&model.texture);
    draw.to_frame(app, &frame).unwrap();
}

fn model(app: &App) -> Model {
    // Create a new window!
    app.new_window()
        .size(HEIGHT, WIDTH)
        .view(view)
        .build()
        .unwrap();
    let img_buf = RgbImage::new(WIDTH, HEIGHT);
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("framebuf.jpg");
    img_buf.save(&img_path).unwrap();
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    Model { texture }
}

fn fill(mut a: RgbImage, config: Config) -> RgbImage {
    let (mut z_bright, mut z, c);
    let (mut fx, mut fy): (f64, f64);
    let (xmax, xmin, ymax, ymin) = (
        2.0 * config.scale_fac,
        -2.0 * config.scale_fac,
        2.0 * config.scale_fac,
        -2.0 * config.scale_fac,
    );
    match config.fractal_type {
        FractalType::Julia => {
            c = Complex::new(config.julia_r, config.julia_i);
        }
        FractalType::Mandelbrot => {
            c = Complex::new(0.0, 0.0);
        }
    }

    for y in 0..config.height {
        fy = y as f64 / config.height as f64 * (ymax - ymin) + ymin;
        if y == config.height / 2 {
            println!("Halfway there!");
        }
        for x in 0..config.width {
            fx = x as f64 / config.width as f64 * (xmax - xmin) + xmin;
            z = Complex::new(fx, fy);
            match config.fractal_type {
                FractalType::Julia => z_bright = julia(z, c).saturating_mul(config.contrast),
                FractalType::Mandelbrot => z_bright = mandelbrot(z).saturating_mul(config.contrast),
            }
            draw_pixel(
                &mut a,
                x,
                y,
                z_bright,
                config.colors,
                config.colors_saturation,
                config.colors_value,
            );
        }
    }
    a
}

fn draw_pixel(
    a: &mut RgbImage,
    x: u32,
    y: u32,
    z_bright: u8,
    colors: bool,
    saturation: f64,
    value: f64,
) {
    match colors {
        true => {
            let pix_color = Color::new_hsl(z_bright as f64, saturation, value).to_rgb_string();
            let pix_output = Color::from_str(&pix_color).unwrap();
            a.put_pixel(
                x,
                y,
                Rgb([pix_output.red, pix_output.green, pix_output.blue]),
            );
        }
        _ => {
            a.put_pixel(x, y, Rgb([z_bright, z_bright, z_bright]));
        }
    }
}

fn mandelbrot(z: Complex<f64>) -> u8 {
    let iterations = 200;
    let mut v: Complex<f64> = Complex::new(0.0, 0.0);
    for n in 0..iterations {
        // MAYBE: Convert this to an iterator for rayon sometime
        v = v.powu(2) + z;
        if v.norm() > 2.0 {
            // return n;
            return n + 1 - (z.norm().ln().log2() as u8);
        };
    }
    255
}

fn julia(mut z: Complex<f64>, c: Complex<f64>) -> u8 {
    let iterations = 200;
    for n in 0..iterations {
        z = z.powu(2) + c;
        if z.norm() >= 2.0 {
            return n + 8 - (z.norm().ln().log2() as u8);
            // return n;
        }
    }
    0
}
