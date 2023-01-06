use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;

fn main() {
    let output_filename = "archive.tar.gz";
    let src_directory = "midjourney";
    let output_path = ".";
    compress_folder(&output_filename, &src_directory, &output_path).unwrap();
}

fn compress_folder(
    output_filename: &str,
    src_directory: &str,
    output_path: &str,
) -> Result<(), std::io::Error> {
    // create an empty tar file
    let tar_gz = File::create(output_filename).unwrap();
    // write midjourney forlder contents into the tar file
    let tar_file = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(tar_file);
    tar.append_dir_all(output_path, src_directory).unwrap();
    Ok(())
}
