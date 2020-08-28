mod student;
use std::collections::HashMap;
use student::{Gender, Student};

fn main() {
    println!("Pushing Students into a map");
    let mut map = HashMap::new();
    Student::add_student(Student::new("Doroti".to_string(), 16, Gender::FEMALE), &mut map);
    println!("{:?}", map);
    println!("--------------");
}
