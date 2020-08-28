mod student;
use std::collections::HashMap;
use student::{Gender, Student};



fn main(){
    let mut students = Student::separate(vec![
        Student::new("Arnold".to_string(), 16, Gender::MALE),
        Student::new("Jack".to_string(), 15, Gender::MALE),
        Student::new("Chuck".to_string(), 17, Gender::MALE),
        Student::new("Doroti".to_string(), 16, Gender::FEMALE),
        Student::new("Melinda".to_string(), 15, Gender::FEMALE),
        Student::new("Claire".to_string(), 17, Gender::FEMALE)
    ]);

    for k in students.keys() {
        println!("-----{:?}-----", k);
        for s in students.get(k).unwrap(){
            println!("{:?}", s);
        }
    }
    println!("--------------");

    Student::increase_age_by_gender(Gender::MALE, &mut students);

    for k in students.keys() {
        println!("-----{:?}-----", k);
        for s in students.get(k).unwrap(){
            println!("{:?}", s);
        }
    }
    println!("--------------");
}