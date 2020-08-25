use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Gender {
    MALE,
    FEMALE,
}

#[derive(Debug)]
pub struct Student {
    name: String,
    age: u32,
    gender: Gender,
}

impl Student {
    pub fn new(name: String, age: u32, gender: Gender) -> Student {
        Student { name, age, gender }
    }

    pub fn separate(students: Vec<Student>) -> HashMap<Gender, Vec<Student>> {
        let mut report: HashMap<Gender, Vec<Student>> = HashMap::new();
        report.insert(Gender::FEMALE, Vec::new());
        report.insert(Gender::MALE, Vec::new());
        for student in students {
            report
                .get_mut(&student.gender)
                .map(|group| group.push(student));
        }
        report
    }
}