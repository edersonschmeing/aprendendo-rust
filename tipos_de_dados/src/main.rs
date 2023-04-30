
use std::mem;

fn main() {
    println!("Tipos e variáveis!");

    let a: u8 = 123; // u = unsigend, 8 bits, 0 - 255.
    println!("a = {}", a); 
    //a = 456;  //variável a é immutable

    // u = unsigend, 0 to 2^n-1. 
    // i = signed, -2^(n-1) .. 2^(n-1)-1  
    // i8 = signed -2^(8-1) = 128 
    // i8 = signed -2^(8-1)-1 = 127        
    let mut b: i8 = 0; // -128 to 127.
    println!("b = {} antes", b);
    b = 42;
    println!("b = {} depois", b);

    let mut c = 123456789; //i32 = 32 bits
    println!("c = {}, ocupa {} bytes", c, mem::size_of_val(&c));
    c = -1; 
    println!("c = {}", c);

    // u8, u16, u32, u64, u128, i8, i16, i32, i64 e i128.

    //usize isize ocupa o tamanho de acordo com a arquitetura.
    let z: isize = 123;
    let tamanho_de_z: usize = mem::size_of_val(&z);
    println!("z = {}, ocupa {} bytes, {}-bits OS", z, tamanho_de_z, tamanho_de_z*8); 
    
    let d: char = 'X';
    println!("{} é um char, tamanho = {} bytes", d, mem::size_of_val(&d));

    //f32 f64 IEEE754 signed!

    let e: f32 = 2.5;
    println!("e = {}, tamanho = {} bytes", e, mem::size_of_val(&e));

    let g: bool = false; // true;
    println!("g = {}, tamanho = {} bytes", g, mem::size_of_val(&g));
    
    
    /* let a: i32 = 42;
    let ref_a: &i32 = &a;
    let b: i32 = *ref_a;
    println!("{}", std::mem::(a));
    
    println!("{}", b);
   */


}