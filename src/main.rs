use gif;
use std::{fs::File, thread, time};

struct AsciiGif {
    text: Vec<String>,
    frame_time: time::Duration,
}

fn print_gif(gif: &AsciiGif) {
    let mut counter: i32 = 0;
    loop {
        println!("\x1b[H{}", gif.text[counter as usize]);
        counter += 1;
        if (counter as usize) > gif.text.len() - 1 {
            counter = 0;
        }
        thread::sleep(gif.frame_time)
        // wait for frame time
    }
}

fn open_gif() {
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let file = File::open("gif/amogus_small.gif").unwrap();

    let mut decoder = decoder.read_info(file).unwrap();
    let mut frames: Vec<String> = vec![];
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        let fixed_frame = fix_gif(frame);
        let lum = conv_frame_lum(fixed_frame);
        let lum = resize_image_simple(&lum, 234, 234, 234, 56);
        frames.push(new_lines(String::from_utf8(conv_lum_char(lum)).unwrap()));
    }
    let out_gif = AsciiGif {
        text: frames,
        frame_time: time::Duration::from_millis(200),
    };
    print_gif(&out_gif)
}

fn new_lines(string: String) -> String {
    string
        .chars()
        .collect::<Vec<char>>()
        .chunks(234)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}
/*
fn resize_image(
    frame: &Vec<u8>,
    in_width: i32,
    in_height: i32,
    out_width: i32,
    out_height: i32,
) -> Vec<u8> {
}
*/

fn resize_image_simple(
    frame: &Vec<u8>,
    in_width: i32,
    in_height: i32,
    out_width: i32,
    out_height: i32,
) -> Vec<u8> {
    let x_factor: f32 = in_width as f32 / out_width as f32;
    let y_factor: f32 = in_height as f32 / out_height as f32;
    let mut new_image = vec![];
    /*for (index, pixel) in new_image.iter_mut().enumerate() {
        *pixel =
    }*/

    for y in 0..out_height {
        for x in 0..out_width {
            // Calculate the corresponding pixel coordinates in the original image
            let original_x: usize = (x as f32 * x_factor) as usize;
            let original_y: usize = (y as f32 * y_factor) as usize;

            // Calculate the indices of the four neighboring pixels in the original image
            let index_tl = original_y * in_width as usize + original_x; // top-left
            let index_tr = index_tl + 1; // top-right
            let index_bl = index_tl + in_width as usize; // bottom-left
            let index_br = index_bl + 1; // bottom-right

            // Calculate the average value of the neighboring pixels
            let avg_value: u8 = ((frame[index_tl] as u16
                + frame[index_tr] as u16
                + frame[index_bl] as u16
                + frame[index_br] as u16)
                / 4) as u8;

            // Push the average value to the resized image
            new_image.push(avg_value);
        }
    }
    return new_image;
}

fn fix_gif(frame: &gif::Frame) -> Vec<[u8; 4]> {
    let mut out: Vec<[u8; 4]> = vec![];
    for pixel in frame.buffer.chunks(4) {
        let mut array: [u8; 4] = [0; 4];
        array.copy_from_slice(pixel);
        out.push(array);
    }
    return out;
}

fn conv_lum_char(frame: Vec<u8>) -> Vec<u8> {
    let ascii_array = ".,-~:;=!*#$@";
    frame
        .iter()
        .map(|elem| ascii_array.as_bytes()[(elem / 23) as usize])
        .collect()
}

// Ok so need to go from some pixles to luminacnce I think
fn conv_frame_lum(frame: Vec<[u8; 4]>) -> Vec<u8> {
    frame
        .iter()
        .filter_map(|chunk| {
            if chunk.len() == 4 {
                let sum: u16 = u16::from(chunk[0]) + u16::from(chunk[1]) + u16::from(chunk[2]);
                Some((sum / 3).try_into().unwrap())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    // Outline:
    // Convert image to correct size
    //      Maybe calculate a good size for image
    // If Black and White (Currently always) convert to luminacnce
    // Convert luminacnce to ascii characters
    // Print out
    // Save to file
    open_gif();
}
