use std::io::{self, Write};

#[derive(PartialEq, Debug)]
enum Class {
    I, II, III, IV, V, VI, VII, VIII, IX, X, NONE
}

#[derive(PartialEq)]
struct Student {
    id: u128,
    name: String,
    class: Class,
    phone: String
}

fn add_student(students: &mut Vec<Student>, student: Student) {
    students.push(student);
}

fn remove_student(students: &mut Vec<Student>, id: u128) {
    students.retain(|student| student.id != id);
}

fn search_student_from_name<'a>(students: &'a [Student], name: &str) -> Vec<&'a Student> {
    students
        .iter()
        .filter(|student| student.name.contains(name))
        .collect()
}

fn search_student_from_class<'a>(students: &'a [Student], class: Class) -> Vec<&'a Student> {
    students
        .iter()
        .filter(|student| student.class == class)
        .collect()
}

fn search_student_from_roll_and_class<'a>(students: &'a [Student], roll: u128, class: Class) -> Option<&'a Student> {
    students
        .iter()
        .find(|student| student.id == roll && student.class == class)
}

fn map_to_class(class: &str) -> Option<Class> {
    match class {
        "1" => Some(Class::I),
        "2" => Some(Class::II),
        "3" => Some(Class::III),
        "4" => Some(Class::IV),
        "5" => Some(Class::V),
        "6" => Some(Class::VI),
        "7" => Some(Class::VII),
        "8" => Some(Class::VIII),
        "9" => Some(Class::IX),
        "10" => Some(Class::X),
        _ => None,
    }
}

fn is_phone_valid(phone: &str) -> bool {
    phone.len() == 10
}

fn manual() {
    println!("========== USER PORTAL ==========");
    println!();
    println!("SELECT THE OPTION TO CONTINUE");
    println!("1. ADD A STUDENT DETAIL");
    println!("2. DELETE A STUDENT DETAIL");
    println!("3. GET A STUDENT INFO");
    println!("4. PRINT ALL STUDENTS")
}

fn search_options_manual() {
    println!("========== SEARCH STUDENT ==========");
    println!();
    println!("1. SEARCH FROM NAME");
    println!("2. SEARCH FROM CLASS");
    println!("2. SEARCH FROM ROLL");
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    let io = io::stdin();

    loop {
        manual();
        let mut input = String::new();
        io.read_line(&mut input).expect("Failed to read input");

        match input.as_str() {
            "1" => {
                let mut id;
                let mut name;
                let mut class;
                let mut phone;
                println!("ENTER STUDENT DETAILS");
                print!("ID: ");
                io.read_line(&mut id);
                print!("NAME: ");
                io.read_line(&mut name);
                print!("CLASS: ");
                io.read_line(&mut class);
                print!("PHONE: ");
                io.read_line(&mut phone);

                let student = Student {
                    id: match id.trim().parse::<u128>() {
                        Ok(num) => num,
                        Err(e) => {
                            writeln!(io::stdout().lock(), "Expected a number for ID").unwrap();
                            0
                        }
                    },
                    name,
                    class: match map_to_class(&class) {
                        Some(class) => class,
                        None => {
                            writeln!(io::stdout().lock(), "Class can only be from 1 to 10").unwrap();
                            Class::NONE
                        }
                    },
                    phone: match is_phone_valid(&phone) {
                        true => phone,
                        false => {
                            writeln!(io::stdout().lock(), "Phone number should be of 10 digits").unwrap();
                            String::from("")
                        }
                    }
                    
                };
                add_student(&mut students, student);       
            },
            
            "2" => {
                
            },

            _ => {
                println!("Not a Valid choice")
            }
        }
        
    }

}


