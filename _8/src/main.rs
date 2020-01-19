#[macro_use(c)]
extern crate cute;

use std::fmt;

mod input;

pub struct Layer {
    pub pixels: Vec<Vec<u32>>,
}

impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tmp: String = "".to_string();
        for line in &self.pixels {
            tmp = format!("{}\n{:?}", tmp, line);
        }
        tmp = tmp.replace(", ", "");
        write!(f, "{}\n", tmp)
    }
}

impl Layer {
    pub fn n_nums_in_layer(&self, num: u32) -> usize {
        // nice to have list comprehension in rust, although in this case it is not optimally used
        // as it instanciates a vector, which is not needed for just counting numbers
        (c!(*p, for p in l, for l in &self.pixels, if *p == num)).len() as usize
    }

    pub fn printable(&self) -> String {
        let mut tmp = format!("{:?}", self);
        tmp = tmp.replace("0", " ");
        tmp = tmp.replace("1", "X");
        tmp
    }
}

#[derive(Debug)]
pub struct SpaceImage {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

impl SpaceImage {
    pub fn from_str_input(width: usize, height: usize, input_str: &str) -> SpaceImage {
        let n_pixels_per_layer = width * height;
        if input_str.len() % n_pixels_per_layer != 0 {
            panic!(
                "Layer size ({}) is not a divider of input size ({})!",
                input_str.len(),
                width * height
            );
        }

        let n_layers = input_str.len() / n_pixels_per_layer;
        let mut si = SpaceImage {
            width: width,
            height: height,
            layers: Vec::with_capacity(n_layers),
        };

        let input_bytes = input_str.as_bytes(); // only numbers, no unicode characters

        // filling of image: nicer would be an iteration over the input string!
        for l in 0..n_layers {
            let mut layer = Layer {
                pixels: Vec::with_capacity(height),
            };
            for i in 0..height {
                let mut line = Vec::with_capacity(width);
                for j in 0..width {
                    let index = l * n_pixels_per_layer + i * width + j;
                    let pixel = (input_bytes[index] as char).to_digit(10).unwrap();
                    line.push(pixel);
                }
                layer.pixels.push(line);
            }
            si.layers.push(layer);
        }

        si
    }

    pub fn merge_down(&self) -> Layer {
        let mut lines = Vec::with_capacity(self.height);
        for i in 0..self.height {
            let mut line = Vec::with_capacity(self.width);
            for j in 0..self.width {
                line.push(self.merge_pixel(i, j));
            }
            lines.push(line);
        }

        Layer { pixels: lines }
    }

    fn merge_pixel(&self, i: usize, j: usize) -> u32 {
        for layer in &self.layers {
            if layer.pixels[i][j] == 2 {
                continue;
            }
            return layer.pixels[i][j];
        }
        2
    }
}

fn main() {
    // task 1: On the layer with the fewest 0's, what's the product of the number of 1's
    // times number of 2's?

    let test = "123456789012";
    let test_img = SpaceImage::from_str_input(3, 2, test);
    println!("TEST PRINT! {:?}", test_img);

    let img = SpaceImage::from_str_input(25, 6, input::PROGRAM_INPUT);

    let (_, layer_with_fewest_zeros) = (0..img.layers.len())
        .map(|i| (img.layers[i].n_nums_in_layer(0), i))
        .min()
        .unwrap();
    let num_1s = img.layers[layer_with_fewest_zeros].n_nums_in_layer(1);
    let num_2s = img.layers[layer_with_fewest_zeros].n_nums_in_layer(2);

    println!("Task 1 result: {:?}", num_1s * num_2s);

    let test = "0222112222120000";
    let test_img = SpaceImage::from_str_input(2, 2, test);
    println!("TEST PRINT! {:?}", test_img.merge_down());

    println!("Task 1 result: {:}", img.merge_down().printable());
}
