use clap::Parser;

mod clipboard;
mod writer;

/// Paste images from your clipboard into a PNG file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output to stdout instead of a file
    #[arg(long)]
    stdout: bool,

    /// The name of the output file
    #[arg(index = 1)]
    name: Option<String>,
}

fn throw(message: &str) {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn main() {
    let args = Args::parse();
    let image = clipboard::read_image();

    match image {
        Ok(image) => match args.stdout {
            true => writer::write_to_stdout(image),
            false => match args.name {
                Some(name) => writer::write_to_file(image, name),
                None => throw("No output file specified"),
            },
        },
        Err(_) => throw("No image found in clipboard"),
    }
}
