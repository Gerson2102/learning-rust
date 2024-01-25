#[derive(Debug)]
pub struct Student {
    name: String,
    age: u16,
    email: String,
}

pub struct StudenList {
    students: Vec<Student>,
}

impl Student {
    pub fn create_student(name: String, age: u16, email: String) -> Student {
        let new_student = Student {
            name: name.to_string(),
            age: age,
            email: email.to_string(),
        };
        return new_student;
    }
}

impl StudenList {
    pub fn new() -> Self {
        StudenList {
            students: Vec::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn get_all_students(&self) -> &Vec<Student> {
        &self.students
    }
}
