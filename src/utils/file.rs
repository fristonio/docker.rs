use std::env;
use std::path::Path;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use uuid::Uuid;

/// This function validates the provided directory path as a string and return the
/// std::path::Path corresponding to argument. It returns the absolute path.
/// It checks for the validity of directory and returns an error if the directory in
/// context does not exist.
/// This will also throw an error if the current directory we are in is not valid.
///
/// ```rust
/// use rust_docker::file::get_validated_dir_path;
/// match get_validated_dir_path("/tmp") {
///     Ok(dir_path) => dir_path,
///     Err(e) => println!("Error occured {}", e)
/// }
/// ```
pub fn get_validated_dir_path(dir: &str) -> Result<Path, String> {
    let mut dir_path = Path::new(dir);

    if !dir_path.is_absolute() {
        match env::current_dir() {
            Ok(cur_dir) => {
                cur_dir.push(dir);
                dir_path = cur_dir.as_path();
            }
            Err(e) => return Err(format!("Current directory not valid {}", e)),
        };
    }

    if !dir_path.is_dir() {
        return Err(format!(
            "The provided directory context is not a valid directory : {}",
            dir
        ));
    }

    Ok(dir_path)
}

/// This function create a GZIpped tarball from the provided directory path
/// it returns with a string error if the Tar could not be formed or if the provided
/// directory is not valid.
///
/// It appends all the files inside the directory recursively to the tar file
/// and write those files to it.
pub fn create_gzipped_tarball(dir: &str) -> Result<Path, String> {
    // This assumes that the base_dir_path is a valid path
    // which is not none when converted to string.
    let base_dir_path = get_validated_dir_path(dir)?;

    let tar_name = format!("{}.tar.gz", Uuid::new_v4().to_simple());

    let tar_file = match File::create(&tar_name) {
        Ok(tar_file) => tar_file,
        Err(err) => {
            return Err(format!(
                "Error while creating tar file : {}",
                err.cause()
            ))
        }
    };

    // Create a new gzip enocder
    let encoder = GzEncoder::new(tar_gz, Compression::default());
    // Create a new tar Builder.
    let mut tar = tar::Builder::new(encoder);

    // Append all files in the directory we want to Tar.
    if let Err(err) = tar.append_dir_all(base_dir_path.to_str().unwrap(), ".") {
        return Err(format!("Error while writing to tar : {}", err.to_string()));
    }

    // Finish building the Tar file
    tar.finish();
    Ok(base_dir_path)
}
