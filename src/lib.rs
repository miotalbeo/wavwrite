use std::fs::File;
use std::io::prelude::*;

pub fn write_wav(filename: String, buf: &mut [u8]) -> bool {
    // WAV FILE LAYOUT
    // // RIFF CHUNK DESCRIPTOR
    // // // Chunk ID       -   4 bytes                 - BIG
    // // // ChunkSize      -   4 bytes                 - LITTLE
    // // // Format         -   4 bytes                 - BIG
    // // FMT SUB-CHUNK
    // // // Subchunk ID    -   4 bytes                 - BIG
    // // // Subchunk Size  -   4 bytes                 - LITTLE
    // // // Audio Format   -   2 bytes                 - LITTLE
    // // // NumChannels    -   2 bytes                 - LITTLE
    // // // SampleRate     -   4 bytes                 - LITTLE
    // // // ByteRate       -   4 bytes                 - LITTLE
    // // // BlockAlign     -   2 bytes                 - LITTLE
    // // // BitsPerSample  -   2 bytes                 - LITTLE
    // // DATA SUB-CHUNK
    // // // Subchunk2 ID   -   4 bytes                 - BIG
    // // // Subchunk2 Size -   4 bytes                 - LITTLE
    // // // data           -   Subchunk2 Size bytes    - LITTLE
    //
    // TO USE:
    // -- Generate wave data as a vector of i8
    // -- For each i8:
    // -- -- Turn into bytes with .to_le_bytes()
    // -- -- Push each byte onto a u8 vector
    // -- -- Pass u8 vector as input to this function

    let buffer_len = buf.len() as i32;

    let file = File::create(filename);

    let mut file = match file {
        Ok(file) => file,
        Err(_) => { return false },
    };

    // RIFF Header
    // Write 'RIFF' into file
    // -- Chunk ID
    if let Err(_) = file.write("RIFF".as_bytes()) {
        return false;
    }

    // Calculate filesize ???
    // -- ChunkSize
    let filesize = buffer_len + 12 + 16 + 8 - 8;
    // Get memory representation of filesize as a byte array 
    // in little-endian
    let filesize = filesize.to_le_bytes();
    if let Err(_) = file.write(&filesize) {
        return false;
    }

    // Write 'WAVE' into file
    // -- Format
    if let Err(_) = file.write("WAVE".as_bytes()) {
        return false;
    }


    // Format chunk
    // -- Subchunk ID
    if let Err(_) = file.write("fmt ".as_bytes()) {
        return false;
    }

    // -- Subchunk Size
    let fmt_len: u32 = 16;
    let fmt_len = fmt_len.to_le_bytes();
    if let Err(_) = file.write(&fmt_len) {
        return false;
    }

    // -- Audio Format
    let audio_format: u16 = 1;
    let audio_format = audio_format.to_le_bytes();
    if let Err(_) = file.write(&audio_format) {
        return false;
    }

    // -- NumChannels
    let num_channels: u16 = 1;
    let num_channels = num_channels.to_le_bytes();
    if let Err(_) = file.write(&num_channels) {
        return false;
    }

    // -- SampleRate
    let sample_rate: u32 = 22050;
    let sample_rate = sample_rate.to_le_bytes();
    if let Err(_) = file.write(&sample_rate) {
        return false;
    }

    // -- ByteRate
    if let Err(_) = file.write(&sample_rate) {
        return false;
    }

    // -- BlockAlign
    let block_align: u16 = 1;
    let block_align = block_align.to_le_bytes();
    if let Err(_) = file.write(&block_align) {
        return false;
    }

    let bits_per_sample: u16 = 8;
    let bits_per_sample = bits_per_sample.to_le_bytes();
    if let Err(_) = file.write(&bits_per_sample) {
        return false;
    }

    // Data chunk
    // -- Subchunk2 ID
    if let Err(_) = file.write("data".as_bytes()) {
        return false;
    }

    // -- Subchunk2 Size
    let subchunk2_size = buffer_len.to_le_bytes();
    if let Err(_) = file.write(&subchunk2_size) {
        return false;
    }

    // -- data
    if let Err(_) = file.write(&buf) {
        return false
    }

    return true;
}
