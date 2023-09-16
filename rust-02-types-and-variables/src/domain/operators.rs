
pub fn demo_operators_bitwise() {

    println!("\noperators bitwise");

    let c = 1 | 2;
    println!("1 | 2 = {}", c); 

     // | (OR) - & (AND)  ^ (XOR)  ! NOT 
    
    let a:i32 = 2;     // Bit 10
    let b:i32 = 3;     // Bit 11
   
    let mut result:i32; 

    result = a & b; //AND Bit 10 
    println!("(a & b) => {} ", result);

    result = a | b; // OR bit 11
    println!("(a | b) => {} ", result) ;

    result = a ^ b; //XOR bit 01
    println!("(a ^ b) => {} ", result);

    result = !b; //NOT 
    println!("(!b) => {} ", result);

    result = !a; //NOT 
    println!("(!a) => {} ", result);

    result = a << b; //LEFT SHIFT
    println!("(a << b) => {}", result);

    result = a >> b; //RIGHT SHIFT
    println!("(a >> b) => {}", result);

}

pub fn demo_operators_arithmetic() {

    println!("\noperators arithmetic");

    let mut a = 2 + 3 * 4; // +-*
    println!("{}", a);

    a = a + 1; //-- ++
    a -= 2; // a = a - 2; 
            // -= += *= /= %=

    let resto_divisao = a % 3;
    println!("Resto da divisão de {} / {} = {},", a, 3, resto_divisao);

    let a_elevado_ao_cubo = i32::pow(a, 3); //potência
    println!("{} elevado a {} = {},", a, 3, a_elevado_ao_cubo);

    let b = 2.5;
    let b_elevado_ao_cubo = f64::powi(b, 3);
    let b_elevado_ao_PI = f64::powf(b, std::f64::consts::PI);
    
    println!("{} elevado a {} = {},", b, 3, b_elevado_ao_cubo);
    println!("{} elevado a PI = {},", b, b_elevado_ao_PI);

}

pub fn demo_operators_logical() {

    println!("\noperators logical");

    let pi_menor_4 =  std::f64::consts::PI < 4.0; //true

    // > < <= >= == != 
   
    let x = 5;  
    let x_igual_5: bool = x == 5; //true
    let x_menor_5 = x < 5; //false
  
    let x_diferente_5 = x != 5; //false

    println!("{ }", x_diferente_5);
     
}
