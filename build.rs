extern crate cc;

use std::path::Path;

fn main() {
    cc::Build::new()
        .file(Path::new("src").join("fiber").join("context.S"))
        .compile("libcontext.a");
}
