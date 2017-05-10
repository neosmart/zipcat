extern crate getopts;
extern crate glob;
extern crate zip;

use getopts::Options;
use glob::{MatchOptions, Pattern};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("s",
                 "silent",
                 "suppress file names from being sent to stderr");
    opts.optmulti("x",
                  "exclude",
                  "exclude file(s) matching pattern (can use more than once)",
                  "PATTERN");
    opts.optmulti("i",
                  "include",
                  "include only file(s) matching pattern (can use more than once)",
                  "PATTERN");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            print!("{}\n", e);
            print_usage(&program, opts, false);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts, true);
        return;
    }

    let parsed = ParsedOptions {
        suppress_file_names: matches.opt_present("s"),
        include_globs: matches.opt_strs("i").iter().map(|str| Pattern::new(str).unwrap()).collect(),
        exclude_globs: matches.opt_strs("x").iter().map(|str| Pattern::new(str).unwrap()).collect(),
    };

    if matches.free.is_empty() {
        print_usage(&program, opts, false);
        return;
    }

    let sources = matches.free;
    for source in sources {
        cat(&source, &parsed).unwrap_or_else(|e| {
            writeln!(&mut std::io::stderr(),
                     "Failed to cat contents of zip file {}: {:?}",
                     source,
                     e)
                .unwrap();
        });
    }
}

fn print_usage(program: &str, opts: Options, include_copyright: bool) {
    use std::path::PathBuf;
    let path = PathBuf::from(program);
    let command = path.file_name().unwrap().to_string_lossy();

    let copyright = "zipcat 0.1 by NeoSmart Technologies. Written by Mahmoud Al-Qudsi \
                     <mqudsi@neosmart.net>";
    println!("Usage: {} ZIPFILE [options]", command);
    print!("Pipes content of compressed file(s) within a zip archive to stdout");
    if include_copyright {
        print!("\n{}", copyright);
    }
    print!("{}", opts.usage(""));
}

struct ParsedOptions {
    suppress_file_names: bool,
    include_globs: Vec<Pattern>,
    exclude_globs: Vec<Pattern>,
}

fn include_file(path: &str, includes: &Vec<Pattern>, excludes: &Vec<Pattern>) -> bool {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    //either (no include filter or matching include filter) + no matching exclude filter
    return (includes.len() == 0 || includes.iter().any(|p| p.matches_with(path, &options))) &&
           !excludes.iter().any(|p| p.matches_with(path, &options));
}

fn cat(source: &str, options: &ParsedOptions) -> Result<()> {
    let file = File::open(source)?;
    let mut zip = zip::ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut z_file = zip.by_index(i)?;
        if !include_file(z_file.name(),
                         &options.include_globs,
                         &options.exclude_globs) {
            continue;
        }

        if !options.suppress_file_names {
            writeln!(&mut std::io::stderr(), "{}: ", z_file.name())?;
        }

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
