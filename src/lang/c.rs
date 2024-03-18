use anyhow::{anyhow, Result};
use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::Command,
};

use super::{Language, LanguageExt};

#[derive(Debug, Default)]
pub struct C;

impl LanguageExt for C {
    const EXT: &'static [&'static str] = &["c", "cpp", "c++", "hpp", "h++"];
}

impl Language for C {
    fn bundle(&self, input: PathBuf, mut output: &mut dyn Write) -> Result<()> {
        Bundler::default().parse_file(&input, &mut output)
    }

    fn build(&self, input: PathBuf, output: Option<PathBuf>) -> Result<()> {
        let mut bundle_file = fs::File::create("nedry_run_tmp.c")?;
        self.bundle(input.clone(), &mut bundle_file as &mut dyn Write)?;
        Command::new("gcc")
            .args(["nedry_run_tmp.c", "-g", "-o"])
            .arg(output.unwrap_or_else(|| "nedry_run_tmp.out".into()))
            .spawn()?
            .wait()?;
        Ok(())
    }

    fn run(&self, input: PathBuf) -> Result<()> {
        self.build(input, None)?;
        Command::new("./nedry_run_tmp.out").spawn()?.wait()?;
        Ok(())
    }
}

#[derive(Debug, Default)]
struct Bundler(HashSet<PathBuf>);

const INCLUDE_TEXT: &str = "#include";

#[derive(Debug, Clone, Copy)]
enum LineModifier {
    Keep,
    Remove,
}

impl Bundler {
    fn parse_file(&mut self, path: &Path, output: &mut impl Write) -> Result<()> {
        if self.0.contains(&path.to_path_buf()) {
            return Ok(());
        }
        self.0.insert(path.to_path_buf());

        // println!("Include file: {}", path.to_string_lossy());
        let file = BufReader::new(fs::File::open(path)?);

        for line in file.lines() {
            let line = line?;
            match self.parse_line(path, line.trim(), output)? {
                LineModifier::Keep => {
                    output.write(line.as_bytes())?;
                    output.write(b"\n")?;
                }
                LineModifier::Remove => {}
            }
        }

        let Some(Some(ext)) = path.extension().map(|s| s.to_str()) else {
            return Ok(());
        };

        let try_include_list: &[&str] = match ext {
            "hpp" | "h++" => &["cpp", "c++"],
            "h" => &["c"],
            _ => return Ok(()),
        };

        for ext in try_include_list {
            let path = path.with_extension(ext);
            if !path.exists() {
                continue;
            }

            self.parse_file(&path, output)?;
            break;
        }

        Ok(())
    }

    fn parse_line(
        &mut self,
        path: &Path,
        line: &str,
        output: &mut impl Write,
    ) -> Result<LineModifier> {
        if !line.starts_with(INCLUDE_TEXT) {
            return Ok(LineModifier::Keep);
        }
        let include = line[INCLUDE_TEXT.len()..].trim_start();
        if !include.starts_with('"') {
            return Ok(LineModifier::Keep);
        }

        let include_relative_path = include.trim_matches('"');
        let include_file_path = path
            .parent()
            .ok_or_else(|| anyhow!("Invalid path (parent of root is not exists)"))?
            .join(Path::new(include_relative_path))
            .canonicalize()?;
        self.parse_file(&include_file_path, output)?;

        Ok(LineModifier::Remove)
    }
}
