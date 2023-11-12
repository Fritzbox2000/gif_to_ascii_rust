use clap::Parser;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, Frame, Frames, ImageDecoder};
use std::{fs::File, thread, time};
/// A simple program for converting gifs to ascii art
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file you want to convert
    #[arg(short, long, required = true)]
    file: String,
    /// Output width of art
    #[arg(short = 'W', long = "Width")]
    width: Option<usize>,
    /// Output height of art
    #[arg(short = 'H', long = "Height")]
    height: Option<usize>,
    /// Whether to print the gif in the terminal or not
    #[arg(short, long, default_value = "false", default_missing_value = "true")]
    print: bool,
    /// Use a different (better for some images) luminance calculator for black and white
    #[arg(short, long, default_value = "false", default_missing_value = "true")]
    luminance: bool,
    /// The time between frames
    #[arg(short, long)]
    time: Option<u64>,
}

type ColourPixel = [u8; 4];

/// Asciigif is for storing ascii versions of gifs
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
        print!(
            "\x1b[H{}",
            (gif.text[counter as usize]).trim_end_matches('\n')
        ); // prints the gif (the first set of
           // characters is the go to the start of console character)
        counter += 1;
        if (counter as usize) > gif.text.len() - 1 {
            counter = 0;
        }
        thread::sleep(gif.frame_time)
    }
}

fn calc_next_frame(
    new_frame_data: &[ColourPixel],
    new_frame_width: u16,
    dist_top: u16,
    dist_left: u16,
    last_frame: &[ColourPixel],
    last_frame_width: u16,
) -> Vec<ColourPixel> {
    // lets create the new frame
    let mut out_frame: Vec<ColourPixel> = last_frame.to_owned();
    let start = (dist_top as usize * (last_frame_width as usize)) + dist_left as usize;
    for (inner_line, pixel) in new_frame_data.iter().enumerate() {
        let index = start
            + (inner_line % (new_frame_width as usize))
            + (last_frame_width as usize * (inner_line / new_frame_width as usize));
        out_frame[index] = *pixel;
    }
    out_frame
}
fn main_loop(args: Args) {
    let mut decoder = open_gif(&args);
    let mut frames: Vec<String> = vec![];

    let (screen_width, screen_height) = get_screen_dimensions();
    let out_width = match args.width {
        Some(x) => x,
        None => screen_width,
    };
    let out_height = match args.height {
        Some(x) => x,
        None => screen_height,
    };

    let mut delay: u16 = 0;
    let mut last_frame: Vec<ColourPixel> = vec![];
    let mut last_frame_width: u16 = 0;
    let mut last_frame_height: u16 = 0;

    let mut gif_decoded: Vec<Frame> = Vec::with_capacity(frames.len());
    for frame in decoder.into_frames() {
        gif_decoded.push(frame.unwrap().clone());
    }

    for frame in gif_decoded.iter() {
        let mut fixed_frame = fix_gif(frame);
        let (width, height) = get_dimensions(frame);
        //delay = frame.delay;
        // if last_frame_height != 0 {
        //     fixed_frame = calc_next_frame(
        //         &fixed_frame,
        //         width,
        //         frame.top,
        //         frame.left,
        //         &last_frame,
        //         last_frame_width,
        //     );
        // } else {
        //     last_frame_height = height;
        //     last_frame_width = width;
        // }
        //
        last_frame = fixed_frame.clone();

        let lum = match args.luminance {
            true => conv_frame_lum(fixed_frame),
            false => conv_frame_lum_2(fixed_frame),
        };

        let resized = resize_image_simple(
            &lum,
            last_frame_width,
            last_frame_height,
            out_width,
            out_height,
        );

        frames.push(new_lines(
            String::from_utf8(conv_lum_char(resized)).unwrap(),
            out_width,
        ));
    }
    let out_gif = AsciiGif {
        text: frames,
        frame_time: time::Duration::from_millis(match args.time {
            Some(x) => x,
            None => delay as u64 * 10,
        }),
    };
    if args.print {
        print_gif(&out_gif)
    }
}
/// This is the main important function at the moment, maybe name should reflect that better?
fn open_gif(args: &Args) -> GifDecoder<File> {
    let file = File::open(args.file.clone()).unwrap();

    GifDecoder::new(file).unwrap()
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

// Ok convolution information
//(Input height + padding height top + padding height bottom - kernel height) / (stride height) + 1
// ( in_h + p - k_h ) / (s + 1) = o
//
fn resize_image_convolution(
    frame: &[u8],
    in_width: i32,
    in_height: i32,
    conv_width: i32,
    conv_height: i32,
) -> Vec<u8> {
    //let new_vec: Vec<u8> = vec![];
    todo!();
}

fn resize_image_simple(
    frame: &[u8],
    in_width: u16,
    in_height: u16,
    out_width: usize,
    out_height: usize,
) -> Vec<u8> {
    // I basically copied this from chatgpt (damn it makes me worse at coding) but its easy
    // I really want to update it. Make is A LOT better, currently it doesn't like it when the
    // output is bigger than the source, plus this method is VERY lossy, basically it will only
    // take a little bit, I wan to take almost everything and average I think, though that might
    // not work
    let x_factor: f32 = in_width as f32 / out_width as f32;
    let y_factor: f32 = in_height as f32 / out_height as f32;
    let mut new_image = Vec::with_capacity(out_width * out_height);

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
    new_image
}

fn fix_gif(frame: &Frame) -> Vec<ColourPixel> {
    // This makes it so each pixel is separate, currently it gets sent
    // [r,g,b,a,r,g,b,a,...,a] I want it to be [[r,g,b,a],[r,g,b,a],...,a]]
    // breaking them up into pixels
    let mut out: Vec<ColourPixel> = Vec::with_capacity(frame.buffer().len() / 4);
    for pixel in frame.buffer().chunks(4) {
        let mut array: ColourPixel = [0; 4];
        array.copy_from_slice(pixel);
        out.push(array);
    }
    out
}

fn conv_lum_char(frame: Vec<u8>) -> Vec<u8> {
    // I'd like to make this a bit better, maybe expand it to use custom character sets etc.
    // another idea is to have a version which just uses the half height box and character color
    // this would double our vertical resolution pretty cool but I probably need to implement the
    // colour stuff.
    let ascii_array = ".,-~:;=!*#$@";
    frame
        .iter()
        .map(|elem| ascii_array.as_bytes()[(elem / 23) as usize])
        .collect()
}

// Some notes the new function _2 is much better, but maybe still offer this function as an option
fn conv_frame_lum(frame: Vec<ColourPixel>) -> Vec<u8> {
    // calculates the average of rgb (it's a little
    // silly but helps when dealing with some images)
    frame
        .iter()
        .map(|chunk| {
            (chunk[0..2].iter().map(|&x| x as u16).sum::<u16>() / 3)
                .try_into()
                .unwrap()
        })
        .collect()
}

fn conv_frame_lum_2(frame: Vec<ColourPixel>) -> Vec<u8> {
    // This is how luminance is ACTUALLY calculated so maybe it should be the default (with the
    // function name at least) it is currently default with the command line arguments
    frame
        .iter()
        .map(|chunk| *chunk[0..2].iter().max().unwrap())
        .collect()
}

fn get_dimensions(frame: &Frame) -> (u16, u16) {
    (
        frame.buffer().width().try_into().unwrap(),
        frame.buffer().height().try_into().unwrap(),
    )
}

fn get_screen_dimensions() -> (usize, usize) {
    term_size::dimensions()
        .expect("Couldn't get the terminal size, please enter output sizes manually")
}

fn main() {
    // Outline:
    // Convert image to correct size
    //      Maybe calculate a good size for image
    // If Black and White (Currently always) convert to luminance
    // Convert luminance to ascii characters
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
    main_loop(args);
}

#[cfg(test)]
mod tests {
    use gif::Encoder;
    use gif::{Frame, Repeat};
    use std::borrow::Cow;
    use std::{fs::File, path::Path};

    use super::*;

    fn create_test_gif(filename: String, test_gif: &Vec<Vec<u8>>) {
        let path_string = &("tests/samples/".to_owned() + &filename);
        let path = Path::new(path_string);
        if path.is_file() {
            return;
        }
        let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
        let (width, height) = (4, 4);
        let mut image = File::create(path_string).unwrap();
        let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();
        for state in test_gif {
            let mut frame = Frame::default();
            frame.width = width;
            frame.height = height;
            frame.buffer = Cow::Borrowed(&*state);
            encoder.write_frame(&frame).unwrap();
        }
    }

    fn standardise_output(gif: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        gif.iter()
            .map(|sub_vector| {
                sub_vector
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| (index + 1) % 4 == 1)
                    .map(|(_, &value)| 1 - (value / 255))
                    .collect()
            })
            .collect()
    }

    #[test]
    fn decode_basic() {
        // create a really small and simple gif
        let test_gif: Vec<Vec<u8>> = vec![
            vec![1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1],
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0],
        ];
        create_test_gif(String::from("test_gif_01.gif"), &test_gif);
        // test it decodes it
        let mut decoder = gif::DecodeOptions::new();
        let mut out: Vec<Vec<u8>> = vec![];
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let file = File::open("tests/samples/test_gif_01.gif").unwrap();
        let mut decoder = decoder.read_info(file).unwrap();
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            out.push(frame.buffer.to_vec());
        }
        out = standardise_output(out);
        assert_eq!(out, test_gif);
    }

    #[test]
    fn decode_compressed() {
        //
        // currently it doesn't encode it with the compression we are looking for
        // so this test is currently useless, I'm not entirely sure what to do about it,
        // other than creating a gif similar to the current test gif manually with the compression
        // we are looking for
        //
        let test_gif = vec![
            vec![1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        ];
        create_test_gif(String::from("test_gif_02.gif"), &test_gif);
        let mut decoder = gif::DecodeOptions::new();
        let mut out: Vec<Vec<u8>> = vec![];
        decoder.set_color_output(gif::ColorOutput::RGBA);
        let file = File::open("tests/samples/test_gif_02.gif").unwrap();
        let mut decoder = decoder.read_info(file).unwrap();
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            out.push(frame.buffer.to_vec());
            println!("{:?}", frame.buffer);
        }
        out = standardise_output(out);
        assert_eq!(out, test_gif)
    }
}
