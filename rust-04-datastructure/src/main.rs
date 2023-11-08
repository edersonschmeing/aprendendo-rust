//https://devoxi.com/rust-struct-dir-pt/

mod domain;
use domain::structures::demo_structures;
use domain::enumerations::demo_enumerations;
use domain::tuples::demo_tuples;
use domain::unions::demo_unions;


fn main() {

   demo_structures();

   demo_enumerations();

   demo_tuples();

   demo_unions();

}