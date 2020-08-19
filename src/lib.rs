use std::f64;
use web_sys::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use num::complex::Complex;

#[wasm_bindgen(start)]
pub fn start() {
    console::log_1(&"Testing".into());

    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let width = canvas.width() as usize;
    let height = canvas.height() as usize;

    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, width, height);
    render_mandelbrot(context, mandelbrot);
}

pub fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(width);

    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);

        for img_x in 0..width {
            let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
            let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64);
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        all_rows.push(row);
    }

    all_rows
}

pub fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }

        z = z * z + c;
    }

    return max_iters;
}

pub fn render_mandelbrot(context: web_sys::CanvasRenderingContext2d, escape_vals: Vec<Vec<usize>>) {
    // web_sys::console::log_2(&"Got mandelbrot values".into(), &escape_vals.into());

    for (i, row) in escape_vals.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            let x = i as f64;
            let y = j as f64;

            let color = match column {
                0..=2 => "red",
                2..=5 => "green",
                5..=10 => "blue",
                11..=30 => "orange",
                30..=100 => "pink",
                100..=200 => "purple",
                200..=400 => "white",
                400..=700 => "silver",
                _ => "black",
            };

            web_sys::console::log_2(&"Fill color: ".into(), &color.into());

            context.set_stroke_style(&color.into());
            context.fill_rect(x, y, 1f64, 1f64);
        } 
    }
}