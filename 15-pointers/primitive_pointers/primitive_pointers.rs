use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

///////////////////////////////////////////////////////////////
// 1. VARIÁVEIS SIMPLES NA STACK
//    - Mutáveis e imutáveis
//    - Acessadas via ponteiros primitivos
///////////////////////////////////////////////////////////////
fn exemplo_stack() {
    println!("--- STACK EXAMPLE COM PONTEIROS PRIMITIVOS ---");

    let x = 10;          // variável imutável
    let mut y = 20;      // variável mutável
    let z: i32 = 30;     // outra imutável

    // Ponteiros primitivos
    let ptr_x: *const i32 = &x;     // ponteiro imutável
    let ptr_y: *mut i32 = &mut y;   // ponteiro mutável
    let ptr_z: *const i32 = &z;

    println!("Endereços das variáveis na stack:");
    println!("  x  = {:p}", &x);
    println!("  y  = {:p}", &y);
    println!("  z  = {:p}", &z);

    unsafe {
        println!("\nLendo valores via ponteiros:");
        println!("  *ptr_x = {}", *ptr_x);
        println!("  *ptr_y = {}", *ptr_y);
        println!("  *ptr_z = {}", *ptr_z);

        println!("\nModificando variável y via ponteiro mutável:");
        *ptr_y = 999;   // permitido
    }

    println!("Novo valor de y = {}", y);

    // OBS: tentar modificar x ou z via ponteiro geraria erro:
    // unsafe { *(ptr_x as *mut i32) = 123; }   // perigoso e UB!
}



///////////////////////////////////////////////////////////////
// 2. ALOCAÇÃO MANUAL NA HEAP (i32) COM PONTEIRO PRIMITIVO
///////////////////////////////////////////////////////////////

fn exemplo_heap_inteiro() {
    println!("\n--- HEAP (i32 usando ponteiro primitivo) ---");

    unsafe {
        let layout = Layout::new::<i32>();

        let ptr = alloc(layout) as *mut i32;

        if ptr.is_null() {
            panic!("falha ao alocar memória!");
        }

        *ptr = 777;

        println!("endereço heap = {:p}", ptr);
        println!("valor = {}", *ptr);

        dealloc(ptr as *mut u8, layout);
    }
}



///////////////////////////////////////////////////////////////
// 3. STRUCT SIMPLES ALOCADA NA HEAP COM PONTEIRO PRIMITIVO
///////////////////////////////////////////////////////////////
#[repr(C)]
struct Pessoa {
    idade: i32,
    altura: f32,
}

fn exemplo_heap_struct() {
    println!("\n--- HEAP (struct usando ponteiro primitivo) ---");

    unsafe {
        let layout = Layout::new::<Pessoa>();

        let ptr = alloc(layout) as *mut Pessoa;

        if ptr.is_null() {
            panic!("falha ao alocar Pessoa!");
        }

        (*ptr).idade = 25;
        (*ptr).altura = 1.80;

        println!("endereço da Pessoa na heap = {:p}", ptr);
        println!("Idade  = {}", (*ptr).idade);
        println!("Altura = {}", (*ptr).altura);

        dealloc(ptr as *mut u8, layout);
    }
} 


///////////////////////////////////////////////////////////////
// DOUBLE POINTER NA HEAP
//  - Primeiro, alocamos um i32 na heap
//  - Depois, alocamos um ponteiro para esse ponteiro, também na heap
//  - Acessamos e modificamos o valor real via níveis de indireção
///////////////////////////////////////////////////////////////
fn ponteiro_de_ponteiro_heap() {
    println!("\n--- DOUBLE POINTER NA HEAP ---");

    // 1) Alocar um i32 na heap
    let layout_i32 = Layout::new::<i32>();
    let ptr_x = unsafe { alloc(layout_i32) as *mut i32 };

    if ptr_x.is_null() { panic!("Falha ao alocar ptr_x"); }

    // Escrever valor
    unsafe { ptr::write(ptr_x, 42); }

    // 2) Alocar um *mut i32 também na heap (um ponteiro para ptr_x)
    let layout_ptr = Layout::new::<*mut i32>();
    let ptr2 = unsafe { alloc(layout_ptr) as *mut *mut i32 };

    if ptr2.is_null() { panic!("Falha ao alocar ptr2"); }

    // ptr2 agora contém ptr_x
    unsafe { ptr::write(ptr2, ptr_x); }

    println!("Endereço do ptr_x (heap)      = {:p}", ptr_x);
    println!("Endereço do ptr2 (heap)       = {:p}", ptr2);

    unsafe {
        println!("Valor via **ptr2              = {}", **ptr2);

        // Modificando o valor real via ponteiro de ponteiro
        **ptr2 = 999;

        println!("Valor modificado de x         = {}", *ptr_x);
    }

    // 3) Liberar memória manualmente
    unsafe {
        dealloc(ptr_x as *mut u8, layout_i32);
        dealloc(ptr2 as *mut u8, layout_ptr);
    }

    println!("Memória liberada corretamente.");
}


fn meu_exemplo() {
    println!("\n--- STACK (usando ponteiro primitivo) ---");
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

///////////////////////////////////////////////////////////////
// FUNÇÃO PRINCIPAL
///////////////////////////////////////////////////////////////
fn main() {
    meu_exemplo();

    exemplo_stack();
    exemplo_heap_inteiro();
    exemplo_heap_struct();

    ponteiro_de_ponteiro_heap();
}



