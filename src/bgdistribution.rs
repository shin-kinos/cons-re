
use std::collections::HashMap;

use crate::weighting::SYMBOL;

pub fn define_bg_dist( arg_b : &String ) -> HashMap<char, f64>
{
	/*
	unsafe {
		println!( "SYMBOL : {:?}", SYMBOL );
	}
	*/

	let mut _dist_list : Vec<f64> = Vec::new();

	match ( *arg_b ).as_str() {
		/* The amino acid order :             A,     R,     N,     D,     C,     Q,     E,     G,     H,     I,     L,     K,     M,     F,     P,     S,     T,     W,     Y,     V */
		"blosum62"  => _dist_list = vec![ 0.078, 0.051, 0.041, 0.052, 0.024, 0.034, 0.059, 0.083, 0.025, 0.062, 0.092, 0.056, 0.024, 0.044, 0.043, 0.059, 0.055, 0.014, 0.034, 0.072 ],
		"swissprot" => _dist_list = vec![ 0.083, 0.056, 0.041, 0.055, 0.014, 0.039, 0.067, 0.071, 0.023, 0.059, 0.097, 0.058, 0.024, 0.039, 0.047, 0.066, 0.054, 0.011, 0.029, 0.069 ],
		"equal"     => _dist_list = vec![ 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050 ],
		_           => _dist_list = vec![ 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050, 0.050 ],
	}

	let mut bg_freq : HashMap<char, f64> = HashMap::new();
	/*
	unsafe {
		for aa in SYMBOL.iter() { bg_freq.insert( *aa, 0.05 ); }
	}
	*/

	unsafe {
		for i in 0 .. SYMBOL.len() {
			bg_freq.insert( SYMBOL[ i ], _dist_list[ i ] );
		}
	}
	//println!( "bg_freq : {:?}", bg_freq );

	bg_freq
}
