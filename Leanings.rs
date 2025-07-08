/*
unwrap() => returns the value of the option
trim() => removes whitespace from both ends of a string

expect() => returns the value of the option or panics with a custom message
unsigned - only positive numbers
signed - positive and negative numbers

~OWNERSHIP~
-single owner 
-refrences dont outlive data they point to
-if value carried , old var is dropped 
-copy types are clones atomatically
 (number ,bool ,char ,tuple)
 -mutable ref allow for read and write
 -only one mut ref at once 

 ~LIFETIME~
 -refrences are valid as long as data they point to is valid
 -every ref has a lifetime
 -annotation -> 'a
(for preventing dangling ref)
-Explicit annotations when complier doesnt understand realtionships 

~Struct~
- eg:
struct Book<'a>{
    title: &'a str,
    author: &'a str,
    year: u16,
}
-struct cannot outlive borrowed data 
STATICLIFETIME -> 'static a
-let S:&`static str = "hello";
(lives for the entire program)

USE #[derive(Debug)] for printing structs and objects (there are no objects in rust)
its a trait

~ENUM~
-allow you to define a type by enumerating its possible values

~Option Enum~
made to handle the concept of nullability 
-enum Option<T>{
    Some(T),
    None,
}

~GENERICS AND TRAIT BOUNDS~
-Generics -> write reusable code
fn first<T>(v:Vec<T>)->T{...}

~TRAITS~
-trait is a collection of methods that can be implemented by a type
-similar to interfaces 
eg : 
trait shape{
fn area(&self)->f64;
}
&&&
impl shape for Circle{
    fn area(&self)->f64{
}

~Annotations~
-used to annotate lifetimes of refrences???
modify compiler (code)behavior  in a declarative way 

~MACROS~
-allow for metaprogramming
-used to write code that generates code at compile time
-diff from fucn in a syntavtic way 
-- declarative way -- replace code
-- procedural way -- generate code , used for code generation and deriving traits 

~common ones~
-println!()
-vec!()
-format!()
-Panic!()
-assert!()

I WILL TALK ABOUT CRATES NOW 
~SERDE~
-used to serialize and deserialize data
-convert one form of data to other 

~BORSH~
-Binary object representation serializer for Hashing 
used to encode and decode data  , (i did it in a array )

~Iterator~
.iter doesnt comsume the data 
.into_iter consumes the data 
*/


fn main() {
    println!("this is all my learnings , still new here , all hand written comments");
}

// fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }