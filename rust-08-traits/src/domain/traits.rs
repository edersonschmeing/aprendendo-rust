trait Animal { 

  fn create(name: &'static str) -> Self; 

  fn name(&self) -> &'static str; 

  fn talk(&self) 
  {
    println!("{}, cannot talk", self.name());
  } 
}

struct Human 
{ 
  name: &'static str
}

struct Cat 
{ 
  name: &'static str
}

impl Animal for Human 
{
  fn create(name: &'static str) -> Human 
  {
    Human { name: name }
  } 

  fn name(&self) -> &'static str 
  {
    self.name
  } 

  fn talk(&self) 
  {
    println!("{}, say hello", self.name());
  }     
}

impl Animal for Cat 
{
  fn create(name: &'static str) -> Cat 
  {
    Cat { name: name }
  } 

  fn name(&self) -> &'static str 
  {
    self.name
  } 

  fn talk(&self) 
  {
    println!("{}, say meow", self.name());
  }     
}

pub fn demo_traits() { 

   println!("\nDemo traits");

   //let h = Human{name: "John"};
   let h:Human = Animal::create("John");
   h.talk();

   let c = Cat{name: "Misty"};
   c.talk();
}