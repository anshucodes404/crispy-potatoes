use std::io::{self, Write};

#[derive(PartialEq, Debug)]
enum Class {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
    OTHER,
}

#[derive(PartialEq, Debug)]
struct Student {
    id: u128,
    name: String,
    class: Class,
    phone: String,
}

fn add_student(students: &mut Vec<Student>, student: Student) {
    students.push(student);
    println!("\nStudent added successfully");
}

fn remove_student(students: &mut Vec<Student>, id: u128, class: Class) {
    students.retain(|student| student.id != id || student.class != class);
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

fn search_student_from_id_and_class<'a>(
    students: &'a [Student],
    id: u128,
    class: Class,
) -> Option<&'a Student> {
    students
        .iter()
        .find(|student| student.id == id && student.class == class)
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

fn read_input(prompt: &str) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    print!("{}", prompt);
    stdout.flush().expect("Failed to flush output");
    stdin.read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

fn read_id(prompt: &str) -> u128 {
    let input = read_input(prompt);

    input.parse::<u128>().unwrap_or_else(|_e| {
        writeln!(io::stdout().lock(), "Expected a number for ID").unwrap();
        0
    })
}

fn read_name(prompt: &str) -> String {
    read_input(prompt)
}

fn read_class(prompt: &str) -> Class {
    let input = read_input(prompt);

    map_to_class(&input).unwrap_or_else(|| {
        writeln!(io::stdout().lock(), "Class can only be from 1 to 10").unwrap();
        Class::OTHER
    })
}

fn read_phone(prompt: &str) -> String {
    let input = read_input(prompt);

    if is_phone_valid(&input) {
        input
    } else {
        writeln!(io::stdout().lock(), "Phone number should be of 10 digits").unwrap();
        String::from("")
    }
}

fn manual() {
    println!("========== USER PORTAL ==========");
    println!();
    println!("SELECT THE OPTION TO CONTINUE");
    println!("1. ADD A STUDENT DETAIL");
    println!("2. DELETE A STUDENT DETAIL");
    println!("3. GET A STUDENT INFO");
    println!("q. QUIT");
}

fn search_options_manual() {
    println!("========== SEARCH STUDENT ==========");
    println!();
    println!("1. SEARCH FROM NAME");
    println!("2. SEARCH FROM CLASS");
    println!("3. SEARCH FROM ID");
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        manual();
        let input = read_input("\nEnter choice: ");

        match input.trim() {
            "1" => {
                println!("ENTER STUDENT DETAILS");
                println!();
                let student = Student {
                    id: read_id("ID: "),
                    name: read_name("NAME: "),
                    class: read_class("CLASS: "),
                    phone: read_phone("PHONE: "),
                };
                add_student(&mut students, student);
            }

            "2" => {
                println!("ENTER DETAILS");

                let id_to_remove = read_id("ID: ");
                let class_to_remove = read_class("CLASS: ");

                remove_student(&mut students, id_to_remove, class_to_remove);
            }

            "3" => {
                search_options_manual();
                let choice = read_input("\nChoose an option to continue: ");

                match choice.as_str() {
                    "1" => {
                        let name = read_name("Enter name: ");
                        let matches = search_student_from_name(&students, &name);

                        if matches.is_empty() {
                            println!("No student found");
                        } else {
                            for student in matches {
                                println!("{:?}", student);
                            }
                        }
                    }
                    "2" => {
                        let class = read_class("Enter class: ");
                        let matches = search_student_from_class(&students, class);

                        if matches.is_empty() {
                            println!("No student found");
                        } else {
                            for student in matches {
                                println!("{:?}", student);
                            }
                        }
                    }
                    "3" => {
                        println!("ENTER DETAILS");
                        let id_to_search = read_id("ID: ");
                        let class_to_search = read_class("CLASS: ");

                        if let Some(student) = search_student_from_id_and_class(
                            &students,
                            id_to_search,
                            class_to_search,
                        ) {
                            println!("{:?}", student);
                        } else {
                            println!("No student found");
                        }
                    }
                    _ => {
                        println!("INVALID CHOICE");
                    }
                }
            }

            "q" => {
                println!("QUTTING THE PROGRAM...");
                break;
            }

            _ => {
                println!("Not a Valid choice")
            }
        }
    }
}
