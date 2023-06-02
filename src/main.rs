use gif;
use std::fs::File;

fn open_gif() {
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let file = File::open("gif/amogus_small.gif").unwrap();

    let mut decoder = decoder.read_info(file).unwrap();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        let lum = conv_frame_lum(frame);
        println!(
            "{}",
            new_lines(String::from_utf8(conv_lum_char(&lum)).unwrap())
        );
        println!("{}", frame.buffer.len());
    }
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

fn conv_lum_char(frame: &Vec<u8>) -> Vec<u8> {
    let ascii_array = ".,-~:;=!*#$@";
    frame
        .iter()
        .map(|elem| ascii_array.as_bytes()[(elem / 23) as usize])
        .collect()
}

// Ok so need to go from some pixles to luminacnce I think
fn conv_frame_lum(frame: &gif::Frame) -> Vec<u8> {
    frame
        .buffer
        .chunks(4)
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
    open_gif();
}
