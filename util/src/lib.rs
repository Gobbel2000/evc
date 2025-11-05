use std::path::PathBuf;
use std::{env, fs, io};

fn find_input_dir() -> io::Result<PathBuf> {
    let cwd = env::current_dir()?;
    cwd.ancestors()
        .map(|p| p.join("input"))
        .find(|p| p.is_dir())
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "No input directory found on any parent",
        ))
}

fn input_file(fname: &str, part: u32) -> io::Result<PathBuf> {
    let dir = find_input_dir()?;
    let direct = dir.join(fname);
    if direct.is_file() {
        Ok(direct)
    } else {
        // Treat 'fname' argument as just quest number
        let part_s = String::from("_p") + &part.to_string();
        let suffix = fname.to_string() + &part_s + ".txt";
        let full_fname = dir
            .read_dir()?
            .find(|der| {
                der.as_ref().is_ok_and(|de| {
                    de.file_type().is_ok_and(|ft| ft.is_file())
                        && de.file_name().to_string_lossy().ends_with(&suffix)
                })
            })
            .ok_or(io::Error::from(io::ErrorKind::NotFound))??
            .file_name();
        Ok(dir.join(full_fname))
    }
}

pub fn get_input(fname: &str) -> io::Result<(String, u32)> {
    let mut part = 1;
    let mut input_path = None;
    for arg in env::args_os().skip(1) {
        if arg == "2" {
            part = 2
        } else if arg == "3" {
            part = 3
        } else if arg != "1" {
            input_path = Some(arg)
        }
    }
    let path = match input_path {
        Some(p) => p.into(),
        None => input_file(fname, part)?,
    };
    let input = fs::read_to_string(path)?;
    Ok((input, part))
}

#[macro_export]
macro_rules! evc_main {
    () => {
        fn input_name() -> String {
            let this_file = ::std::file!();
            let path = ::std::path::Path::new(this_file);
            let quest = path.file_stem().unwrap();
            quest.to_str().unwrap().to_owned()
        }
        util::evc_main!(&input_name());
    };
    ( $x:expr ) => {
        fn main() -> std::io::Result<()> {
            let (input, part) = util::get_input($x)?;
            match part {
                1 => part1(input),
                2 => part2(input),
                3 => part3(input),
                other => panic!("Not a puzzle part: {other}")
            }
            Ok(())
        }
    };
}
