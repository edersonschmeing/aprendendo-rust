fn main() { 

    let a = 123; 
    println!("a = {} ", a);

    let a = 777;  
    println!("a = {} ", a); //shadowing

    { 
       let b = 456;
       println!("b = {} ", b);
       println!("a = {} ", a);

       let a = 555;   //shadowing
       println!("a = {} ", a);
        
    }
    println!("a = {} ", a); 
    //println("b = {} ", b); //fora do escopo.

}