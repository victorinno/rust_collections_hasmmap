use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[warn(dead_code)]
pub enum Gender {
    MALE,
    FEMALE,
}

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: u32,
    gender: Gender,
}

impl Student {
    pub fn new(name: String, age: u32, gender: Gender) -> Student {
        Student { name, age, gender }
    }

    #[warn(dead_code)]
    fn initialize_gender(gender: Gender, map: &mut HashMap<Gender, Vec<Student>>) {
        map.entry(gender).or_insert(Vec::new());
    }

    #[warn(dead_code)]
    pub fn add_student(student: Student, map: &mut HashMap<Gender, Vec<Student>>) {
        let students = map.entry(student.gender.clone()).or_insert(Vec::new());
        students.push(student);
    }

    #[warn(dead_code)]
    pub fn try_to_modify() {
        let mut change: HashMap<Gender, Vec<Student>> = HashMap::new();
        Student::initialize_gender(Gender::FEMALE, &mut change);
        change
            .get_mut(&Gender::FEMALE)
            .map(|g| g.push(Student::new("Arnold".to_string(), 16, Gender::MALE)));
        println!("{:?}", change);
        Student::initialize_gender(Gender::FEMALE, &mut change);
        println!("{:?}", change);
    }

    #[warn(dead_code)]
    pub fn separate(students: Vec<Student>) -> HashMap<Gender, Vec<Student>> {
        let mut report: HashMap<Gender, Vec<Student>> = HashMap::new();
        Student::initialize_gender(Gender::FEMALE, &mut report);
        Student::initialize_gender(Gender::MALE, &mut report);
        for student in students {
            report
                .get_mut(&student.gender)
                .map(|group| group.push(student));
        }
        report
    }

    pub fn increase_age_by_gender(gender: Gender, students: &mut HashMap<Gender, Vec<Student>>) {
        let selecteds: &mut Vec<Student> = students.entry(gender).or_default();
        for student in selecteds {
            student.age += 1;
        }
    }
}
