fn main() { 

    let a = 123; 
    println!("a = {} ", a);
  
    { 
       let a = 555;   
       println!("a = {} ", a);
    }
    
    println!("a = {} ", a); 

}