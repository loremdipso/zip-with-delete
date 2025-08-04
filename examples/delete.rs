use std::{fs::OpenOptions, path::PathBuf, str::FromStr};

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
    let to_delete = args[2..].iter().collect::<Vec<_>>();

    let zip_editor = zip::ZipEditor::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(archive)
            .unwrap(),
    )
    .unwrap();

    zip_editor
        .delete_files(&to_delete)
        .expect("Couldn't delete files");

    0
}
