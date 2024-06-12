/// The program reads the file list, checks if the source directory and
/// target directory exist, makes a list of every file in the source
/// directory recursively, and then copies any file from the source
/// to the target if the file is in the file list.
///
/// By default, the program will print the files that would be copied.
/// To copy the files for real, use the `--disable-dry-run` flag.
///
/// Example usage:
/// ```
/// $ cargo run -- --file_list files.txt --source_dir /mnt/a --target_dir /mnt/b
/// ```
///
/// Note: This program assumes that the file list, source directory, and target
/// directory are valid and accessible. If any of these paths do not exist, the
/// program will print an error message and exit.
///
/// More information can be found in the command line help message.
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path;
use std::path::Path;
use walkdir::WalkDir;

/// Finder copies files from a list of file names to a destination directory.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a file containing a list of file names to copy.
    #[arg(short, long)]
    file_list: String,

    /// Directory where the files are located.
    #[arg(short, long)]
    source_dir: String,

    /// Directory where the files will be copied.
    #[arg(short, long)]
    target_dir: String,

    /// Disable dry run mode, copy files for real.
    #[arg(short, long, action)]
    disable_dry_run: bool,
}

fn main() {
    // Parse the command line arguments.
    let args = Args::parse();

    // Get the path to the file list.
    let file_name_list_path = args.file_list;

    // Stop if the file list does not exist.
    if !Path::new(&file_name_list_path).exists() {
        println!(
            "ERROR: Path to file list `{}` does not exist",
            file_name_list_path
        );
        return;
    }

    // Read the file list.
    let reader = BufReader::new(File::open(file_name_list_path).expect("ERROR: Cannot open file."));

    // Store the file names in a vector.
    let mut file_names = Vec::new();
    for line in reader.lines() {
        let line = line.expect("ERROR: Cannot read line.");
        file_names.push(line);
    }

    // Get the absolute path of the source directory.
    let source_directory = args.source_dir;
    let absolute_source = match path::absolute(source_directory) {
        Ok(p) => p,
        Err(e) => panic!("ERROR: Problem with file: {:?}", e),
    };
    let absolute_source_string: String = absolute_source.display().to_string();

    // Stop if the source directory does not exist.
    if !Path::new(&absolute_source).exists() {
        println!(
            "ERROR: Source path `{}` does not exist",
            absolute_source_string
        );
        return;
    }

    // Read the files in the source directory into a hashmap.
    println!("Reading files from: {}", absolute_source_string);
    let mut source_files = HashMap::new();

    for entry in WalkDir::new(absolute_source)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let file_name = String::from(entry.file_name().to_string_lossy());
        let full_path = String::from(entry.path().to_string_lossy());
        // Deduplicate file names.
        println!("Inserting: `{}`", &file_name);
        source_files.insert(file_name, full_path);
    }

    // Get the absolute path of the destination directory.
    let target_directory = args.target_dir;
    let absolute_target = match path::absolute(target_directory) {
        Ok(p) => p,
        Err(e) => panic!("ERROR: Problem with file: {:?}", e),
    };
    let absolute_target_string: String = absolute_target.display().to_string();

    // Stop if the destination directory does not exist.
    if !Path::new(&absolute_target).exists() {
        println!(
            "ERROR: Target path `{}` does not exist",
            absolute_target_string
        );
        return;
    }

    // Stop if the destination directory is not empty.
    if absolute_target.read_dir().unwrap().next().is_some() {
        println!(
            "ERROR: Target path `{}` is not empty",
            absolute_target_string
        );
        return;
    }

    // Copy the files to the destination directory.
    let disable_dry_run = args.disable_dry_run;
    for (file_name, full_path) in source_files.into_iter() {
        if file_names.contains(&file_name) {
            let target_path = format!("{}/{}", absolute_target_string, file_name);
            if disable_dry_run {
                println!("Copying `{}` to `{}`", full_path, target_path);
                fs::copy(full_path, target_path).expect("Cannot copy file.");
            } else {
                println!("DRY RUN. Not copying `{}` to `{}`", full_path, target_path);
            }
        }
    }
}
