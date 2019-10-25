use std::fs::{create_dir, remove_dir_all};
use std::path::Path;

fn main() {
    let check_path = Path::new("/tmp/tantivy");
    let dir_exists = check_path.exists();
    if dir_exists {
        remove_dir_all(check_path).expect("dir does not exist");
    }

    let index_path = Path::new("/tmp/tantivy");
    create_dir(index_path).expect("dir already exists");
}
