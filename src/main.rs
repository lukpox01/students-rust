fn main() {
    let mut students: Vec<Student> = Vec::new();
    loop {
        let mut student_index = manage_students_menu(&mut students);
        if student_index == 0 {
            break
        }
        student_index -= 1;
        manage_student_menu(&mut students, student_index);
        
    }
}


fn manage_students_menu(students: &mut Vec<Student>) -> usize{
    return loop {
        println!("MENU");
        println!("1) add student");
        println!("2) select student");
        println!("q) exit");
        let mut i = String::new();
        std::io::stdin().read_line(&mut i).unwrap();
        if i.trim() == "1" {
            let mut name = String::new();
            let mut age = String::new();
            println!("Name:");
            std::io::stdin().read_line(&mut name).unwrap();
            println!("Age:");
            std::io::stdin().read_line(&mut age).unwrap();
            let age: i8 = age.trim().parse().unwrap();
            students.push(Student::new(name.trim(), age));
        } else if i.trim() == "2" {
            for (idx, student) in students.iter().enumerate() {
                println!("{}) {}", idx + 1, student.name)
            }
            let mut student_idx = String::new();
            std::io::stdin().read_line(&mut student_idx).unwrap();
            let student_idx: usize = student_idx.trim().parse().unwrap();
            break student_idx;
        } else if i.trim() == "q" {
            break 0;
        }
    };
}

fn manage_student_menu(students: &mut Vec<Student>, student_index: usize){
    loop {
        println!("{}-{} MENU", students[student_index].name, students[student_index].age);
        println!("1) add grade");
        println!("2) remove grade");
        println!("3) show grades");
        println!("q) exit");
        let mut i = String::new();
        std::io::stdin().read_line(&mut i).unwrap();
        if i.trim() == "1" {
            let mut subject = String::new();
            let mut grade = String::new();
            println!("Subject:");
            std::io::stdin().read_line(&mut subject).unwrap();
            println!("Grade:");
            std::io::stdin().read_line(&mut grade).unwrap();
            let grade: i8 = grade.trim().parse().unwrap();

            students[student_index].grades.add_grade( &subject.trim(), grade);
        } else if i.trim() == "2" {
            let mut subject = String::new();
            let mut grade = String::new();
            println!("Subject:");
            std::io::stdin().read_line(&mut subject).unwrap();
            println!("Grade:");
            std::io::stdin().read_line(&mut grade).unwrap();
            let grade: i8 = grade.trim().parse().unwrap();

            students[student_index].grades.remove_grade(&subject.trim(), grade);
        } else if i.trim() == "3" {
            let mut subject = String::new();
            println!("Subject:");
            std::io::stdin().read_line(&mut subject).unwrap();
            println!(
                "{}{:?}",
                subject.trim(),
                students[student_index].grades.get_grades(&subject.trim())
            );
        } else if i.trim() == "q" {
            break;
        }
    }
}
struct Grades {
    sjl: Vec<i8>,
    anj: Vec<i8>,
    mat: Vec<i8>,
}

impl Grades {
    fn new() -> Grades {
        Grades {
            sjl: Vec::new(),
            anj: Vec::new(),
            mat: Vec::new(),
        }
    }

    fn get_grades(&self, subject: &str) -> Vec<i8> {
        match subject {
            "sjl" => self.sjl.clone(),
            "anj" => self.anj.clone(),
            "mat" => self.mat.clone(),
            _ => Vec::new(),
        }
    }

    fn add_grade(&mut self, subject: &str, grade: i8) {
        match subject {
            "sjl" => self.sjl.push(grade),
            "anj" => self.anj.push(grade),
            "mat" => self.mat.push(grade),
            _ => println!("doesnt work"),
        }
    }

    fn remove_grade(&mut self, subject: &str, grade: i8) -> Option<i8> {
        match subject {
            "sjl" => Some(
                self.sjl
                    .remove(self.sjl.iter().position(|&x| x == grade).unwrap()),
            ),
            "anj" => Some(
                self.anj
                    .remove(self.sjl.iter().position(|&x| x == grade).unwrap()),
            ),
            "mat" => Some(
                self.mat
                    .remove(self.sjl.iter().position(|&x| x == grade).unwrap()),
            ),
            _ => None,
        }
    }
}

struct Student {
    name: String,
    age: i8,
    grades: Grades,
}

impl Student {
    fn new(name: &str, age: i8) -> Student {
        Student {
            name: String::from(name),
            age: age,
            grades: Grades::new(),
        }
    }
}
