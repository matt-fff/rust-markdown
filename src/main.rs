use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Parserating {}", _filename);

    // Create a path variable from the filename
    let inpath = Path::new(_filename);
    let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");
    let mut outfile =
        File::create(output_filename.to_string()).expect("[ ERROR ] Could not create output file!");

    // Try to open the file
    let file = File::open(&inpath).expect("[ ERROR ] Failed to open file!");

    let mut _ptag: bool = false; // keep track of paragraph tags
    let mut _htag: bool = false; // keep track of h1 tags
    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap().to_string();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n"); // adding \n for instructional clarity
                }
                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n"); // close it if we're already open
                }

                _htag = true;
                output_line.push_str("<h1>");
                output_line.push_str(&line_contents[2..]); // Get all but the first two characters
            }

            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&line_contents);
            }
        };
        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }
        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }
        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }
    println!("[ INFO ] Parserating completely deetely");
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    let mut banner = String::new();
    banner.push_str("\nCopy Cat Coders: ");
    banner.push_str(env!("CARGO_PKG_AUTHORS"));
    banner.push_str("\nRepositizzle: ");
    banner.push_str(env!("CARGO_PKG_HOMEPAGE"));
    banner.push_str("\nUseizzle: ");
    banner.push_str(env!("CARGO_PKG_NAME"));
    banner.push_str(" <whatever_file_you_want_its_your_life>.md");
    println!("{}", banner)
}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage();
        }
    }
}
