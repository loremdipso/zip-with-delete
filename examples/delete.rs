use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    str::FromStr,
};
use zip::write::SimpleFileOptions;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!(
            "Usage: {} <existing archive> <filenames to delete>",
            args[0]
        );
        return 1;
    }

    let existing_archive_path = &*args[1];
    let archive = PathBuf::from_str(existing_archive_path).unwrap();
    let to_append = args[2..]
        .iter()
        .map(|arg| PathBuf::from_str(arg).unwrap())
        .collect::<Vec<_>>();

    let mut append_zip = zip::ZipEditor::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(archive)
            .unwrap(),
    )
    .unwrap();

    append_zip.finish().unwrap();

    0
}
