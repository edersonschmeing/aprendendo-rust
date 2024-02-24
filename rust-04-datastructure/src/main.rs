//https://devoxi.com/rust-struct-dir-pt/

mod domain;
use domain::structures::demo_structures;
use domain::enumerations::demo_enumerations;
use domain::tuples::demo_tuples;
use domain::unions::demo_unions;
use domain::arrays::demo_arrays;
use domain::options::demo_options;
use domain::slices::demo_slices;
use domain::pattern_matching::demo_pattern_matching;




fn main() {

   demo_structures();

   demo_enumerations();

   demo_unions();

   demo_options();

   demo_arrays();

   demo_slices();

   demo_tuples();

   demo_tuples();
   
   demo_pattern_matching();
   

}