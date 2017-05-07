extern crate getopts;
extern crate zip;

use getopts::Options;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    }

    let sources = matches.free;
    for source in sources {
        cat(&source).unwrap_or_else(|e| {
            writeln!(&mut std::io::stderr(),
                     "Failed to cat contents of zip file {}: {:?}",
                     source,
                     e)
                .unwrap();
        });
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} ZIPFILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn cat(source: &str) -> Result<()> {
    let mut file = File::open(source)?;
    let mut zip = zip::ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut z_file = zip.by_index(i)?;
        writeln!(&mut std::io::stderr(), "{}: ", z_file.name())?;

        let mut stdout = std::io::stdout();
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = z_file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            stdout.write_all(&buffer[0..bytes_read])?;
        }
    }

    Ok(())
}
