# Zip (but with delete, too!)

Fixes [#166](https://github.com/zip-rs/zip2/issues/166).

Eventually the goal is to get this crate cleaned up and merged into the mainline zip crate. I'm making this public now since, well, I need it, but if you find this and aren't afraid of untested code then you're welcome to use this too :)

## The idea

Since the [zip file format](https://en.wikipedia.org/wiki/ZIP_(file_format)) can have arbitrary data embedded in it, I took the approach of conserving whatever data I could. So rather than relying on a zip file's purported size I assume that all of the bytes between the start of a file's header and the start of the next file's header are the file's data. So if a zip file has junk before editing then that junk will be maintained after, which in practice I've found to be the most stable.

## Usage

```rust
use zip::write::{UpdateZip};

let mut zip_file = zip::ZipWriter::new_editor(
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(archive)
        .expect("Can't open file")
).expect("Can't edit zip");

let filenames_to_delete = vec!["01.txt", "02.txt"];
zip_file.delete_files(&filenames_to_delete).expect("Issue deleting files");
```

## Performance

tl;dr - basically instant :) When deleting multiple files this copies the minimum amount of data. Rather than shifting the entire remainder of the file over to cover up holes this does what you would do intuitively, and just shifts until the next hole.

Back-of-the-envelope, on the examples I was able to get my hands on this takes <10ms to remove 10 files from an archive with 500 files. Smaller archives are done so fast it's a little difficult to measure. Doing a more traditional copy-all-the-files-except-the-ones-I-want-to-delete takes on the order of ~200ms or so, on my machine. Even the worst case, where you delete nearly all of the files, is faster than creating a new archive (for me).

## Stability

I'm happy with how stable this is, but as you can see I haven't written any tests. Use at your own risk!
