use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use gnuplot::{Figure, Caption, Color};
use std::error::Error;
use csv;
use rand::Rng;
use std::convert::TryInto;

// Provided function for string to Vec<i32> conversion
fn string_to_vec_i32(input: &str) -> Vec<i32> {
    input.chars().filter_map(|c| c.to_digit(10)).map(|num| num as i32).collect()
}

// Provided function for loading an array from a file
pub fn load_array_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<f64>>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let nums: Vec<f64> = buffer
        .chunks_exact(8)
        .map(|chunk| f64::from_ne_bytes(chunk.try_into().unwrap()))
        .collect();
    let n = nums.len() / 400; // Adjust 'n' as needed
    let arrays = (0..n).map(|i| nums[i*400..(i+1)*400].to_vec()).collect();
    Ok(arrays)
}

// Provided function for creating a plot
fn create_plot(data: Vec<f64>, file_name: &str) -> Result<(), Box<dyn Error>> {
    let x: Vec<usize> = (0..data.len()).collect();
    let y = data;
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&x, &y, &[Caption("Data"), Color("red")]);
    fg.set_terminal("png", file_name);
    fg.show()?;
    Ok(())
}
pub fn filter_samples(samples: &Vec<Vec<f64>>, genome: &Vec<i32>) -> Vec<Vec<f64>> {
    samples.iter().map(|epoch| {
        epoch.iter().enumerate().map(|(i, &value)| {
            if i < genome.len() && genome[i] == 0 { 0.0 } else { value }
        }).collect()
    }).collect()
}

// Main function to execute the described process
fn process_files(csv_path: &str, binary_path: &str) -> Result<(), Box<dyn Error>> {
    // Read CSV and find genome with highest floating-point number
    let mut rdr = csv::Reader::from_path(csv_path)?;
    let mut highest_score = std::f64::MIN;
    let mut genome = String::new();

    for result in rdr.records() {
        let record = result?;
        let score: f64 = record.get(0).unwrap().parse()?;
        if score > highest_score {
            highest_score = score;
            genome = record.get(1).unwrap().to_string();
        }
    }

    // Convert genome to Vec<i32>
    let genome_vec = string_to_vec_i32(&genome);

    // Load array from binary file
    let mut arrays = load_array_from_file(binary_path)?;

    //arrays = filter_samples(&arrays.clone(), &genome_vec); 

    // Select a random Vec<f64>
    let mut rng = rand::thread_rng();
    let random_index = 0;//rng.gen_range(0..arrays.len());
    let selected_array = &arrays[random_index];

    // Generate plot
      // Extract filenames from paths
    let csv_file_name = Path::new(csv_path)
        .file_name()
        .ok_or("Could not extract file name from CSV path")?
        .to_str()
        .ok_or("Could not convert CSV file name to string")?;

    let binary_file_name = Path::new(binary_path)
        .file_name()
        .ok_or("Could not extract file name from binary path")?
        .to_str()
        .ok_or("Could not convert binary file name to string")?;

    // Generate the plot file name by combining the two filenames
    let plot_file_name = format!("{}_{}.png", csv_file_name, binary_file_name);

    // Use `plot_file_name` for the output file name in the plotting function
    create_plot(selected_array.clone(), &plot_file_name)?;

    Ok(())
}

fn main() {
	let file_names = vec![
		"../savedIter/iteration_0_4_hz.csv",
		"../savedIter/iteration_12_16_hz_maybe.csv",
		"../savedIter/iteration_16_20_hz.csv",
		"../savedIter/iteration_20_24_hz.csv",
		"../savedIter/iteration_24_28_hz.csv",
		"../savedIter/iteration_4_8_hz.csv",
		"../savedIter/iteration_8_12_hz.csv",
	];

	let file_names_t2 = vec![
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T2_0.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T2_1.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T2_2.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T2_3.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T2_4.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T2_5.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T2_6.bin",
	];
	let file_names_t1 = vec![
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T1_0.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T1_1.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T1_2.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T1_3.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T1_4.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T1_5.bin",
		"../board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T1_6.bin",
	];

	for i in 0..file_names.len() {
		if let Err(e) = process_files(file_names[i], file_names_t1[i]) {
			println!("Error: {}", e);
		}
	}
}

