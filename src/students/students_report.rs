mod student;
use student::{Student, Gender};

fn main() {
    let report = Student::separate(vec![
        Student::new("Arnold".to_string(), 16, Gender::MALE),
        Student::new("Jack".to_string(), 15, Gender::MALE),
        Student::new("Chuck".to_string(), 17, Gender::MALE),
        Student::new("Doroti".to_string(), 16, Gender::FEMALE),
        Student::new("Melinda".to_string(), 15, Gender::FEMALE),
        Student::new("Claire".to_string(), 17, Gender::FEMALE)
    ]);
    for k in report.keys() {
        println!("-----{:?}-----", k);
        for s in report.get(k).unwrap(){
            println!("{:?}", s);
        }
    }
    println!("--------------");
}
