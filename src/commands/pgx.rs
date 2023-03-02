use std::path::Path;
use std::include_str;


pub fn build_pgx(path: &Path, output_path: &str) {
    // your code for building a pgx extension goes here
    println!("Building pgx extension at path {}", &path.display());
    let dockerfile = include_str!("./pgx_builder/Dockerfile");
    println!("{}", dockerfile);

}
