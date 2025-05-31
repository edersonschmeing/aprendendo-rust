mod domain;
use domain::primitive_pointers::demo_primitive_pointers;
use domain::vector_primitive_pointers::demo_vector_primitive_pointers;
use domain::vector_box_primitive_pointers::demo_vector_box_primitive_pointers;

fn main() {
    demo_primitive_pointers();

    demo_vector_primitive_pointers();

    demo_vector_box_primitive_pointers();

}

