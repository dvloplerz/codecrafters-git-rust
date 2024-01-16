#[allow(unused_imports)]
use flate2::{self, read::ZlibDecoder};
use std::env::args;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = args().collect();

    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git")
                .and_then(|_| fs::create_dir(".git/objects"))
                .and_then(|_| fs::create_dir(".git/refs"))
                .and_then(|_| fs::write(".git/HEAD", "ref: refs/heads/master\n"))?;
            println!("Initialized git directory");
        }
        "cat-file" => {
            if &args[2] == "-p" {
                let git_object_path = ".git/objects";
                let mut raw_content = String::new();
                let (path, file) = &args[3].split_at(2);
                let file_location = Path::new(&git_object_path).join(path).join(file);
                if file_location.exists() {
                    ZlibDecoder::new(File::open(file_location)?)
                        .read_to_string(&mut raw_content)?;
                    let prettyfy_content = raw_content
                        .split_once("\0")
                        .unwrap()
                        .1
                        .trim_end()
                        .trim_end();
                    print!("{}", prettyfy_content.trim_end())
                };
            }
        }
        _ => {
            println!("unknown command: {}", args[1])
        }
    };
    Ok(())
}
