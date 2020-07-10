#[macro_use]
extern crate serde_derive;

pub mod types;
use types::Tensor;
mod runtime;

use getopts::Options;
use image::{FilterType, GenericImageView};
use ndarray::Array;
use std::{collections::HashMap, env, fs::File, io::BufReader};

const IMG_HEIGHT: usize = 224;
const IMG_WIDTH: usize = 224;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "c",
        "ms-backend-config",
        "set wasm backend config file",
        "FILE_PATH",
    );
    opts.optopt(
        "i",
        "input-data-file",
        "set the path to input image file",
        "FILE_PATH",
    );
    opts.optopt(
        "l",
        "label-class-file",
        "set the path to label class file",
        "FILE_PATH",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let wasm_backend_file: String = match matches.opt_str("c") {
        Some(s) => s,
        None => String::from(""),
    };
    let input_data_file: String = match matches.opt_str("i") {
        Some(s) => s,
        None => String::from(""),
    };
    let label_class_file: String = match matches.opt_str("l") {
        Some(s) => s,
        None => String::from(""),
    };
    let img = image::open(input_data_file).unwrap();
    let input = data_preprocess(img);

    let output: Tensor = match runtime::execute(wasm_backend_file, input) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    output_assert(output, label_class_file);
}

fn data_preprocess(img: image::DynamicImage) -> Tensor {
    println!("original image dimensions: {:?}", img.dimensions());
    let img = img
        .resize_exact(IMG_HEIGHT as u32, IMG_WIDTH as u32, FilterType::Nearest)
        .to_rgb();
    println!("resized image dimensions: {:?}", img.dimensions());
    let mut pixels: Vec<f32> = vec![];
    for pixel in img.pixels() {
        let tmp = pixel.data;
        // normalize the RGB channels using mean, std of imagenet1k
        let tmp = [
            (tmp[0] as f32 - 123.0) / 58.395, // R
            (tmp[1] as f32 - 117.0) / 57.12,  // G
            (tmp[2] as f32 - 104.0) / 57.375, // B
        ];
        for e in &tmp {
            pixels.push(*e);
        }
    }

    // (H,W,C) -> (C,H,W)
    let arr = Array::from_shape_vec((IMG_HEIGHT, IMG_WIDTH, 3), pixels).unwrap();
    let arr = arr.permuted_axes([2, 0, 1]);
    let arr = Array::from_iter(arr.into_iter().map(|&v| v));

    return Tensor::from(arr);
}

fn output_assert(out_tensor: Tensor, label_class_file: String) {
    let output = out_tensor.to_vec::<f32>();

    // Find the maximum entry in the output and its index.
    let mut argmax = -1;
    let mut max_prob = 0.;
    for i in 0..output.len() {
        if output[i] > max_prob {
            max_prob = output[i];
            argmax = i as i32;
        }
    }

    // Create a hash map of (class id, class name)
    let mut synset: HashMap<i32, String> = HashMap::new();
    let mut rdr = csv::ReaderBuilder::new().from_reader(BufReader::new(
        File::open(label_class_file.as_str()).unwrap(),
    ));

    for result in rdr.records() {
        let record = result.unwrap();
        let id: i32 = record[0].parse().unwrap();
        let cls = record[1].to_string();
        synset.insert(id, cls);
    }

    println!(
        "input image belongs to the class `{}`",
        synset
            .get(&argmax)
            .expect("cannot find the class id for argmax")
    );
}
