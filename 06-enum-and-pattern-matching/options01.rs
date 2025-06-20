fn main() {
   let x: Option<i32> = Some(5);
   let y: Option<i32> = None;

   match x {
       Some(v) => println!("Valor: {}", v),
       None => println!("Sem valor"),
   }
}