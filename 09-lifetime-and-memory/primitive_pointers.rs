pub fn demo_primitive_pointers() {
    
    println!("\n Demo Primitive Pointers");

    let x = 42;
    let ptr_x: *const i32 = &x; // ponteiro imutável

    unsafe {
        println!("Valor via ponteiro: {}", *ptr_x);
    }

    let mut y = 10;
    let ptr_y: *mut i32 = &mut y; // ponteiro mutável

    unsafe {
        *ptr_y = 99;
        println!("Novo valor de y: {}", y);
    }
      
}