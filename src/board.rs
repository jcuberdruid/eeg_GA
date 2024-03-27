use rand::Rng; use lazy_static::lazy_static;
use std::sync::Arc;
use std::{fmt, cmp};
use clustering::*;
//use rand::*;

mod eegHandler;

#[derive(Clone, Default)]
pub struct Board {
    pub queens: Vec<i32>
}

lazy_static! {
    pub static ref EEG_DATA_1: Arc<Vec<Vec<f64>>> = { // needs to actually take an int to define the file to open for T1 but later 
        Arc::new(eegHandler::load_array_from_file("board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T1_1.bin").unwrap()) //0-4 -> 1
    };
    pub static ref EEG_DATA_2: Arc<Vec<Vec<f64>>> = {
        Arc::new(eegHandler::load_array_from_file("board/subepochData_80_top_5_channels/subepoched_filtered_MI_RLH_T2_1.bin").unwrap())
    };

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str: String = String::new();

        for num in &self.queens[0..self.queens.len() - 1] {
            str.push_str(&num.to_string());
            str.push_str(", ");
        }

        str.push_str(&self.queens[self.queens.len() - 1].to_string());
        write!(f, "{}", str)
    }
}

pub fn filter_samples(samples: &Vec<Vec<f64>>, genome: &Vec<i32>) -> Vec<Vec<f64>> {
    samples.iter().map(|epoch| {
        epoch.iter().enumerate().map(|(i, &value)| {
            if i < genome.len() && genome[i] == 0 { 0.0 } else { value }
        }).collect()
    }).collect()
}

//pub fn fitness(board: &Board) -> f64 { old 
pub fn fitness(genome: &Vec<i32>) -> f64 { // fitness function for EEG
    // Directly reference the global data without cloning
    let og_samples_1 = &*EEG_DATA_1;
    let og_samples_2 = &*EEG_DATA_2;

    // Apply genome filter to all subepochs
    let samples_1 = filter_samples(og_samples_1, genome);
    let samples_2 = filter_samples(og_samples_2, genome);

    // Combine samples_1 and samples_2
    let mut samples = samples_1.clone();
    samples.extend(samples_2.iter().cloned());

    let n_samples = samples.len();
    //println!("n_samples {}", n_samples);
    
    //println!("{}{}{}{}", genome[0], genome[1], genome[2], genome[3]);

    let sample_0 = samples[0].clone();
    //println!("{:#?}", sample_0);

    //println!("n_dimensions {}", n_dimensions);
    //println!("n_dimensions {}", n_dimensions);

    let k = 2;
    let max_iter = 200; //need to experiment
    
    //do clustering
    let clustering = kmeans(k, &samples, max_iter);
	    
    let clustered_membership = clustering.membership;
    //println!("{:#?}", clustered_membership);
    let mut num_of_first_cluster = 0;
    let mut num_of_second_cluster = 0;
    for x in clustered_membership {
            if x == 0 {
                    num_of_first_cluster +=1;
            } else {
                    num_of_second_cluster +=1;
            }
    }
    
    let smallest = cmp::min(num_of_first_cluster, num_of_second_cluster) as f64;
    let fitness = (smallest/((n_samples as f64)/2.0)) as f64;
    //print!("({})", fitness);
    print!("({:.4})", fitness);
    return fitness;
}

// generate neighbors 
pub fn generate_neighboards(board: &Board) -> Vec<Board>{
    let mut rng = rand::thread_rng();
    let n = board.queens.len();
    let mut neighboards:Vec<Board> = vec![];

    for i in 0..n {
        let mut this_move: Board = board.clone();
        this_move.queens[i] = rng.gen_range(1..=8);
        neighboards.push(this_move);
        /*
        Encountered plateuing, changed random moves
        let row = board.queens[i];
        for amount in 1..(std::cmp::max(row - 1, (n as i32) - row)) {
            if row + amount < (n as i32) {
                this_move.queens[i] = row + amount;
                neighboards.push(this_move.clone());
            }
            if row > amount {
                this_move.queens[i] = row - amount;
                neighboards.push(this_move);
            }
        }
        */
    }
    return neighboards; 
}


