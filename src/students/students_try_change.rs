mod student;
use student::{Student, Gender};

fn main() {
    println!("trying to change");
    Student::try_to_modify();
    println!("--------------");
}
