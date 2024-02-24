use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::{load_from_memory};
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn  grayscale(encoded_file: &str) -> String {
log(&"Grayscale Called".into()); 
let base64_to_vector = decode(encoded_file).unwrap();
log(&"Image decode".into());
let mut img = load_from_memory(&base64_to_vector).unwrap();
log(&"Image Loaded".into());
img = img.grayscale();
log(&"Grayscale applied".into());
let mut buffer = vec![];
img.write_to(&mut buffer, Png).unwrap();
log(&"New image written".into());

let encode_image = encode(&buffer);
let data_url = format!(
    "data:img/png;base64,{}",
    encode_image
);
    data_url
}