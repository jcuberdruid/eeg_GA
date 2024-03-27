#![allow(dead_code)]
#![allow(unused_variables)]

use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;
use rand::Rng;

mod board;
mod geneticAlgorithm;

extern crate gnuplot;

use gnuplot::{Figure, Caption, Color};
use crate::gnuplot::*;

fn create_plot(data: Vec<f64>, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let x: Vec<usize> = (0..data.len()).collect();
    let y = data;

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&x, &y, &[Caption("Data"), Color("red")]);

    fg.set_terminal("png", file_name);
    fg.show()?;

    Ok(())
}

fn string_to_vec_i32(input: &str) -> Vec<i32> {
    input.chars().filter_map(|c| c.to_digit(10)).map(|num| num as i32).collect()
}


fn main() {
/*
    let data: Vec<i32> = string_to_vec_i32("1101111101100010011010001010100010000100111000010100110010111011011110000001001001110011001110101000001110110000100000011010011010100110001101010011001010110010001010010000010010000010010010010011000101101010101010001111001100111101010001000011010010000100010110001101111000010010101010100001100100111000110000111011011100001001111111110011101101001001101100011011110011001111001000000011111110110100");

    println!("{:#?}", data);
    let mut dataVec: Vec<Vec<i32>> = Vec::new();
    dataVec.push(data.clone());
    let mut filtered_waveforms_1: Vec<Vec<f64>> = Vec::new();
    let mut filtered_waveforms_2: Vec<Vec<f64>> = Vec::new();

    let og_samples_1 = &*board::EEG_DATA_1;
    let og_samples_2 = &*board::EEG_DATA_2;

    filtered_waveforms_1.append(&mut board::filter_samples(og_samples_1, &data));
    filtered_waveforms_2.append(&mut board::filter_samples(og_samples_2, &data));

    //let result = create_plot( filtered_waveforms_1[1024].clone(), &"class_1_2_8_12.png");
    //let result = create_plot( filtered_waveforms_2[2000].clone(), &"class_2_2_8_12.png");
    let fitness = board::fitness(&data);
    println!("sanity check {}", fitness);
    use std::process::exit;
    exit(0);
*/
    //println!("hello");
    //let mut queens: Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8].to_vec();
    let mut queens: Vec<i32> = Vec::new();
    for _ in 0..400 {
	queens.push(rand::thread_rng().gen_range(0..=1));
    }
    //println!("{:#?}", queens);

    let mut rng: StepRng = StepRng::new(2, 13);
    let mut irs: Irs<i32> = Irs::default();
    let _ = irs.shuffle(&mut queens, &mut rng);

    let test_board = board::Board{queens};
    print!("seed fitness ");
    let fitness = board::fitness(&test_board.queens);
    println!("");
    let possible_moves = board::generate_neighboards(&test_board);
    //println!("{}",possible_moves.len());

    geneticAlgorithm::genetic_algorithm(test_board, 3000, 50,  5,  0.005);
}
