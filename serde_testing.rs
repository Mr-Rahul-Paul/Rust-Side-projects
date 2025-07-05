use serde::{Deserialize as he, Serialize};
use serde_json::{self , Value};

#[derive(Debug , Serialize , he)]
struct Person {
    name :String , 
    age : u32 , 
}

fn main() {
    let person = Person{
        name : String::from("hello ji") ,
        age :30 ,
    };
    
    // // Serialize the person to JSON
    let json_str = serde_json::to_string(&person).unwrap(); 
    println!("Person as JSON: {}", json_str);

  let deser_person : Person = serde_json::from_str(&json_str).unwrap();
  println!("Deserialized person: {:?}", deser_person);
}
