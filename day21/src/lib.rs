use std::collections::HashMap;

pub mod parser;
pub use parser::parse_rule;

pub type Pixel = bool;
pub type Image = Vec<Vec<Pixel>>;
pub type Rule = (Image, Image);
pub type Rules = HashMap<Image, Image>;

/// Base image from which all images are derived
pub fn base() -> Image {
    vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ]
}

/// produce the image which is the input rotated 90 degrees clockwise
fn rotate(image: &Image) -> Image {
    let mut out = vec![vec![false; image.len()]; image.len()];
    for in_row in 0..image.len() {
        for in_col in 0..image.len() {
            let out_row = in_col;
            let out_col = image.len() - in_row - 1;
            out[out_row][out_col] = image[in_row][in_col];
        }
    }
    out
}

fn flip_v(image: &Image) -> Image {
    let mut out = vec![vec![false; image.len()]; image.len()];
    for in_row in 0..image.len() {
        for col in 0..image.len() {
            let out_row = image.len() - in_row - 1;
            out[out_row][col] = image[in_row][col];
        }
    }
    out
}

fn flip_h(image: &Image) -> Image {
    let mut out = vec![vec![false; image.len()]; image.len()];
    for row in 0..image.len() {
        for in_col in 0..image.len() {
            let out_col = image.len() - in_col - 1;
            out[row][out_col] = image[row][in_col];
        }
    }
    out
}

fn permute(image: &Image) -> Vec<Image> {
    let mut out = Vec::with_capacity(12);
    let mut image = image.clone();
    for _ in 0..4 {
        out.push(image.clone());
        out.push(flip_v(&image));
        out.push(flip_h(&image));
        image = rotate(&image);
    }
    out
}

pub fn generate_rules(rules: &[Rule]) -> Rules {
    let mut out = Rules::new();
    for &(ref source, ref dest) in rules {
        for source_permutation in permute(source) {
            out.insert(source_permutation, dest.clone());
        }
    }
    out
}

pub fn enhance(rules: &Rules, image: &Image) -> Image {
    // images are always square
    let size = image.len();
    let (in_chunk_size, out_chunk_size, out_size) = if size % 2 == 0 {
        // enhance 2-chunks
        (2, 3, (size / 2) * 3)
    } else {
        // rules should enforce that this is true
        assert!(size % 3 == 0);
        (3, 4, (size / 3) * 4)
    };
    let mut out = vec![vec![false; out_size]; out_size];

    for row in 0..(size / in_chunk_size) {
        for col in 0..(size / in_chunk_size) {
            // row, col are chunk indices
            let mut in_chunk: Image = Vec::with_capacity(in_chunk_size);
            for ic_row_idx in 0..in_chunk_size {
                let mut in_chunk_row = Vec::with_capacity(in_chunk_size);
                in_chunk_row.extend(image[
                        (row * in_chunk_size) + ic_row_idx
                    ][
                        (col*in_chunk_size)..((col+1)*in_chunk_size)
                    ].iter());
                in_chunk.push(in_chunk_row);
            }

            let out_chunk = rules.get(&in_chunk).expect("No rule found for input chunk");
            for out_c_r in 0..out_chunk_size {
                for out_c_c in 0..out_chunk_size {
                    out[(row * out_chunk_size) + out_c_r][(col * out_chunk_size) + out_c_c] =
                        out_chunk[out_c_r][out_c_c];
                }
            }
        }
    }

    out
}
