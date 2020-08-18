use io::Read;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::hash::BuildHasherDefault;
use std::io;
use std::time::Instant;
use structopt::StructOpt;
use twox_hash::XxHash64;

/// Find any duplicate files in given folder and it's subfolders and delete them
#[derive(StructOpt)]
struct Cli {
    /// Don't delete duplicate files, just list them
    #[structopt(short, long)]
    dry: bool,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    if args.dry {
        println!("dry run");
    }
    let start = Instant::now();
    let mut database: HashMap<Vec<u8>, String, BuildHasherDefault<XxHash64>> = Default::default();
    let (todelete, scanned) = read_directory(args.path, &mut database)?;
    println!("Found {} duplicates", todelete.len());
    for name in &todelete {
        if !args.dry {
            println!("X Deleting: {}", name);
            fs::remove_file(name)?;
        } else {
            println!("Duplicate file: {}", name);
        }
    }
    println!("DONE");
    println!("Scanned {} files in {:?}", scanned, start.elapsed());
    if !args.dry {
        println!("Deleted {} files.", todelete.len());
    }
    Ok(())
}

fn read_directory(
    path: std::path::PathBuf,
    database: &mut HashMap<Vec<u8>, String, BuildHasherDefault<XxHash64>>,
) -> Result<(Vec<String>, u32), Box<dyn Error>> {
    let mut todelete = Vec::<String>::new();
    let mut scanned: u32 = 0;
    println!("reading {:?}", path);
    for entry in fs::read_dir(path)? {
        scanned += 1;
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        if metadata.is_file() {
            let mut file = fs::File::open(&path)?;
            let name = path.to_str().unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            match database.insert(buffer, format!("{}", name)) {
                Some(_) => {
                    todelete.push(name.to_string());
                }
                None => (),
            }
        } else {
            let (more_to_delete, more_scanned) = &mut read_directory(path, database)?;
            scanned = scanned + *more_scanned;
            todelete.append(more_to_delete);
        }
    }
    Ok((todelete, scanned))
}
