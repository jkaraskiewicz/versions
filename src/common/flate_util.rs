use super::{errors::VersionsError, stream_util};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

pub fn flate_directory<P: AsRef<Path>>(
    input_directory_path: P,
    output_file_path: P,
) -> Result<(), VersionsError> {
    let content = stream_util::stream_dir(input_directory_path.as_ref())?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(content.as_bytes())?;
    let compressed_data = encoder.finish()?;

    fs::remove_file(output_file_path.as_ref()).unwrap_or_default();
    let mut file = File::create_new(output_file_path)?;
    file.write_all(&compressed_data)?;
    Ok(())
}

pub fn deflate_directory<P: AsRef<Path>>(
    input_file_path: P,
    output_directory_path: P,
) -> Result<(), VersionsError> {
    let decompressed_data = deflate_to_string(input_file_path)?;
    println!("decompressed = {}", decompressed_data);
    println!("path = {:?}", output_directory_path.as_ref());
    stream_util::destream_dir(&decompressed_data, output_directory_path.as_ref())?;
    Ok(())
}

pub fn deflate_to_string<P: AsRef<Path>>(input_file_path: P) -> Result<String, VersionsError> {
    let file = File::open(input_file_path)?;

    let mut decoder = GzDecoder::new(file);
    let mut decompressed_data = String::new();

    decoder.read_to_string(&mut decompressed_data)?;

    Ok(decompressed_data)
}
