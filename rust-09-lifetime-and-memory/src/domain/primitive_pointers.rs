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

    //Criando e usando ponteiros manualmente
    let x = Box::new(5); // aloca no heap
    let raw_ptr: *const i32 = Box::into_raw(x); // Aqui usamos Box::into_raw para obter um ponteiro bruto

    unsafe {
        println!("Valor via ponteiro: {}", *raw_ptr);
        // recupere a propriedade do Box (evita vazamento de memória)
        let _x = Box::from_raw(raw_ptr as *mut i32); //depois Box::from_raw para retomar o controle do valor
    }

   
}