
use std::collections::HashMap;
use std::f64;

use crate::bgdistribution;
use crate::weighting::SYMBOL;

pub fn relative_entropy( site_list : &Vec<String>, weight_list : &Vec<f64>, gap_pen_list : &Vec<f64>, arg_b : &String ) -> Vec<f64>
{
	unsafe { 
		/* 20 symbols to calculate relative entropy, ignoring gaps. */
		SYMBOL.clear();
		SYMBOL = "ARNDCQEGHILKMFPSTWYV".chars().collect();
		SYMBOL.shrink_to_fit();
		//println!( "{:?}", SYMBOL );
	}

	let num_site : usize = ( *site_list ).len();

	/* Back ground frequency */
	let bg_freq : HashMap<char, f64> = bgdistribution::define_bg_dist( arg_b );

	let mut re_list : Vec<f64> = vec![ 0.0; num_site ];

	for i in 0 .. num_site {
		let re : f64 = cal_re( &( *site_list )[ i ], weight_list, ( *gap_pen_list )[ i ], &bg_freq );
		re_list[ i ] += re;
	}

	re_list
}

fn cal_re( site_arg : &String, weight_list : &Vec<f64>, gap_penalty : f64, bg_freq : &HashMap<char, f64> ) -> f64
{
	let site : Vec<char> = ( *site_arg ).chars().collect();
	//println!( "site : {:?}", site );

	/* Calculate frequency in a site taking account of sequence weighting. */
	let weighted_freq : HashMap<char, f64> = weighted_freq_count( &site, weight_list );

	/* Calculate sum of weighting scores ( denominator of the probability ). */
	let mut sum_weight : f64 = 0.0;
	unsafe {
		for aa in SYMBOL.iter() {
			sum_weight += weighted_freq[ aa ];
		}
	}
	//println!( "Sum of weighting scores ignoring gaps( denominator of the probability ) : {:.3}", sum_weight );

	/* Calculate Relative entropy */
	let mut re : f64 = 0.0;
	unsafe {
		for aa in SYMBOL.iter() {
			let prob : f64 =  weighted_freq[ aa ] / sum_weight;
			re +=  prob * ( prob / ( *bg_freq )[ aa ] ).ln();
		}
	}

	re = re / ( 20 as f64 ).ln();

	/* Give the gap penalty */
	re = re * gap_penalty;

	//println!( "\nRelative entropy : {:.3}\n", re );

	re
}

fn weighted_freq_count( site : &Vec<char>, weight_list : &Vec<f64> ) -> HashMap<char, f64>
{
	let len_site : usize = ( *site ).len();

	/* Define the pseudocount (10e-8). */
	let pseudo_count : f64 = 0.0000001;

	/* Define a hashmap to count AA frequency in a site. */
	let mut freq : HashMap<char, f64> = HashMap::new();
	unsafe {
		for aa in SYMBOL.iter() { freq.insert( *aa, pseudo_count ); }
	}
	//println!( "{:?}", freq );

	/*
	 * Count a frequency of each AA in a site taking accont of sequence weighting.
	 * Add a weighting score instead of simple increment (+1.0).
	 * !!! Gaps are ignored !!!
	 * aa               = One letter of AA in a site.
	 * add              = Weighting score add instead of 1.0.
	 * weight_list[ i ] = Weighting score of i th sequence.
	 * freq             = AA - weighted frequency hashmap.
	*/
	for i in 0 .. len_site {
		let aa  : char = ( *site )[ i ];
		if aa != '-' {
			let add : f64  = freq[ &aa ] + ( *weight_list )[ i ];
			freq.insert( aa, add );
		}
	}
	//println!( "Frequency : {:?}", freq );

	freq
}