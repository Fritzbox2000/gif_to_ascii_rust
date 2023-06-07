use clap::Parser;
use gif;
use std::{fs::File, path::PathBuf, thread, time};
use term_size;

/// A simple program for converting gifs to ascii art
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file you want to convert
    #[arg(short, long)]
    file: String,
    /// Output width of art
    #[arg(short, long)]
    width: Option<usize>,
    /// Output height of art
    #[arg(short = 'H', long)]
    height: Option<usize>,
}

/// Ascigif is for storing acsii versions of gifs
struct AsciiGif {
    /// This is how we are going to store the gif when it has been converted, this
    /// should be used for stuff like saving the gif and playing it back and stuff
    text: Vec<String>,
    frame_time: time::Duration,
}
/// For showing gifs
fn print_gif(gif: &AsciiGif) {
    let mut counter: i32 = 0; // counter for which frame we are on
    loop {
        // repeat until ctrl-c'd, this should be upgraded to be a bit nicer perhaps?
        // TODO: Upgrade loop to work a bit nicer in cmd prompt
        println!("\x1b[H{}", gif.text[counter as usize]); // prints the gif (the first set of
                                                          // characters is the go to the start of console character)
        counter += 1;
        if (counter as usize) > gif.text.len() - 1 {
            counter = 0;
        }
        thread::sleep(gif.frame_time)
        // wait for frame time
    }
}
/// This is the main important function at the moment, maybe name should reflect that better?
fn open_gif(args: Args) {
    // A bunch of setup code
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    let file = File::open(args.file).unwrap(); // TODO: let user set filename

    let mut decoder = decoder.read_info(file).unwrap(); // Some more setup
    let mut frames: Vec<String> = vec![];

    // Handle output size
    let (out_width, out_height) = get_screen_dimensions();

    // size of the gif (this m)

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        //println!("size 0: {}", frame.buffer.len());
        let fixed_frame = fix_gif(frame);

        let (width, height) = get_dimensions(frame);
        println!("{}", width * height);

        //println!("size 1: {}", fixed_frame.len());
        let lum = conv_frame_lum(fixed_frame);
        //println!("size 2: {}", lum.len());
        let resized = resize_image_simple(
            &lum,
            width as i32,
            height as i32,
            out_width as i32,
            out_height as i32,
        );
        //println!("size 3: {}", lum.len());
        frames.push(new_lines(
            String::from_utf8(conv_lum_char(resized)).unwrap(),
            out_width,
        ));
    }
    let out_gif = AsciiGif {
        text: frames,
        frame_time: time::Duration::from_millis(200),
    };
    print_gif(&out_gif)
}

fn new_lines(string: String, width: usize) -> String {
    string
        .chars()
        .collect::<Vec<char>>()
        .chunks(width)
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
    //println!("{}", x_factor);
    let y_factor: f32 = in_height as f32 / out_height as f32;
    //println!("{}", y_factor);
    let mut new_image = vec![];

    // println!("Frame size {}", frame.len());
    // println!("Input est size {}", in_width * in_height);
    for y in 0..out_height {
        for x in 0..out_width {
            // Calculate the corresponding pixel coordinates in the original image
            let original_x: usize = (x as f32 * x_factor) as usize;
            let original_y: usize = (y as f32 * y_factor) as usize;

            // Calculate the indices of the four neighboring pixels in the original image
            let index_tl = (original_y * in_height as usize) + original_x; // top-left
            let index_tr = index_tl + 1; // top-right
            let mut index_bl = index_tl + in_height as usize; // bottom-left
            let mut index_br = index_bl + 1; // bottom-right
            if index_bl > frame.len() {
                index_bl = index_tl;
            }
            if index_br > frame.len() {
                index_br = index_tr;
            }
            //println!("{} | {} | {} | {}", index_tl, index_tr, index_bl, index_br);
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

fn get_dimensions(frame: &gif::Frame) -> (usize, usize) {
    return (frame.width as usize, frame.height as usize);
}

fn get_screen_dimensions() -> (usize, usize) {
    if let Some((w, h)) = term_size::dimensions() {
        return (w, h);
    } else {
        panic!("Couldn't get the terminal size, please enter in output sizes manually");
    }
}

fn main() {
    // Outline:
    // Convert image to correct size
    //      Maybe calculate a good size for image
    // If Black and White (Currently always) convert to luminacnce
    // Convert luminacnce to ascii characters
    // Print out
    // Save to file
    // OK now lets write a TODO
    // I want this to be runnable from cmd
    // I need arguments for this then
    // set the image from path
    // set the speed of the gif
    // different downsampling methods
    //
    let args = Args::parse();
    open_gif(args);
}
