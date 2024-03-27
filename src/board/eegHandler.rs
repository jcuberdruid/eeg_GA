use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn load_array_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<f64>>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    
    // Read the entire file into a buffer.
    file.read_to_end(&mut buffer)?;
    
    // Convert bytes to f64s.
    let nums: Vec<f64> = buffer
        .chunks_exact(8)
        .map(|chunk| f64::from_ne_bytes(chunk.try_into().unwrap()))
        .collect();
    
    // Assuming you know the 'n' (number of samples)
    // and each Vec<f64> should have 10240 elements,
    // split the flat Vec<f64> accordingly.
    let n = nums.len() / 400; // Adjust 'n' as needed
    let arrays = (0..n).map(|i| nums[i*400..(i+1)*400].to_vec()).collect();
    
    Ok(arrays)
}
