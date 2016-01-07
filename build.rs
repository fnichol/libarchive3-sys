extern crate pkg_config;

fn main() {
    match pkg_config::find_library("libarchive") {
        Ok(_) => (),
        Err(msg) => panic!("Unable to locate libarchive, err={:?}", msg),
    }
}
