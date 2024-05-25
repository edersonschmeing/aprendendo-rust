mod domain;
use domain::functions::demo_functions;
use domain::methods::demo_methods;
use domain::closures::demo_closures;
use domain::higher_order_functions::demo_higher_order_functions;



fn main() {

    demo_functions();
    
    demo_methods();

    demo_closures();

    demo_higher_order_functions();
}
