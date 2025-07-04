// pub enum option<T> {
//     Some(T),
//     None,
// }

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
            //works // println!("{}" ,index as i32);
            // println!("{}" ,Some(index as i32)); doesntt work
        }
    }
    return None;
}
fn main() {
    let my_string = String::from("rahul paul");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}
