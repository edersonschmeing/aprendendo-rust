mod domain;

use domain::core_data_types::demo_core_data_types;
use domain::operators::demo_operators_bitwise;
use domain::operators::demo_operators_arithmetic;
use domain::operators::demo_operators_logical;

fn main() {
    
    demo_core_data_types();

    demo_operators_bitwise();

    demo_operators_arithmetic();
    
    demo_operators_logical();
    
}
