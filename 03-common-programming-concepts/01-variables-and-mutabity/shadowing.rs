fn main() { 

    let a = 123; 
    println!("a = {} ", a);

    let a = 777;  
    println!("a = {} ", a); //shadowing

    let a = 555;   //shadowing
    println!("a = {} ", a);

}