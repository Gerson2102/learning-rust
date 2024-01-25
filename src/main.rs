use std::{io, process::exit};

pub mod calculator_functions;
pub mod closures;
pub mod modules;
pub mod optiontest;
pub mod structs;
fn main() {
    println!("Hello, world!");
    // modules::helper::greet();
    // closures::closures();
    // let result = optiontest::test_option();
    // println!("{}", result.unwrap());
    // calculator();
    let student1 = structs::Student::create_student(
        "Gerson".to_string(),
        22,
        "geloaiza@estudiantec.cr".to_string(),
    );
    let student2 = structs::Student::create_student(
        "Daniel".to_string(),
        21,
        "daniel@estudiantec.cr".to_string(),
    );
    let student3 = structs::Student::create_student(
        "Josue".to_string(),
        21,
        "josue@estudiantec.cr".to_string(),
    );
    let mut student_list = structs::StudenList::new();
    student_list.add_student(student1);
    student_list.add_student(student2);
    student_list.add_student(student3);

    let all_students = student_list.get_all_students();

    for student in all_students {
        println!("{:?}", student);
    }
}

#[allow(dead_code)]
fn calculator() {
    println!("=====================================================");
    println!("Welcome to my basic calculator!");
    println!("Here are the options:");
    println!("1. Add numbers");
    println!("2. Substract numbers");
    println!("3. Multiply numbers");
    println!("4. Divide numbers");
    println!("5. Exit\n");

    println!("Now choose :)");

    let mut user_option = String::new();

    io::stdin()
        .read_line(&mut user_option)
        .expect("An error occurred taking the user option!");

    if user_option.trim() == "1" {
        println!("\nEnter the first number: ");
        let mut string_number_1 = String::new();
        io::stdin()
            .read_line(&mut string_number_1)
            .expect("An error occurred taking the first number!");

        println!("\nEnter the second number: ");
        let mut string_number_2 = String::new();
        io::stdin()
            .read_line(&mut string_number_2)
            .expect("An error occurred taking the second number!");

        let number_1: i64 = string_number_1.trim().parse().unwrap();
        let number_2: i64 = string_number_2.trim().parse().unwrap();
        let result = calculator_functions::operations::add(number_1, number_2);
        println!();
        print!("Adding {} + {} = {}", number_1, number_2, result);
        println!("\n\n");
        calculator();
    } else if user_option.trim() == "2" {
        println!("\nEnter the first number: ");
        let mut string_number_1 = String::new();
        io::stdin()
            .read_line(&mut string_number_1)
            .expect("An error occurred taking the first number!");

        println!("\nEnter the second number: ");
        let mut string_number_2 = String::new();
        io::stdin()
            .read_line(&mut string_number_2)
            .expect("An error occurred taking the second number!");

        let number_1: i64 = string_number_1.trim().parse().unwrap();
        let number_2: i64 = string_number_2.trim().parse().unwrap();
        let result = calculator_functions::operations::substract(number_1, number_2);
        println!();
        print!("Substracting {} - {} = {}", number_1, number_2, result);
        println!("\n\n");
        calculator();
    } else if user_option.trim() == "3" {
        println!("\nEnter the first number: ");
        let mut string_number_1 = String::new();
        io::stdin()
            .read_line(&mut string_number_1)
            .expect("An error occurred taking the first number!");

        println!("\nEnter the second number: ");
        let mut string_number_2 = String::new();
        io::stdin()
            .read_line(&mut string_number_2)
            .expect("An error occurred taking the second number!");

        let number_1: i64 = string_number_1.trim().parse().unwrap();
        let number_2: i64 = string_number_2.trim().parse().unwrap();
        let result = calculator_functions::operations::multiply(number_1, number_2);
        println!();
        print!("Multiplying {} * {} = {}", number_1, number_2, result);
        println!("\n\n");
        calculator();
    } else if user_option.trim() == "4" {
        println!("\nEnter the first number: ");
        let mut string_number_1 = String::new();
        io::stdin()
            .read_line(&mut string_number_1)
            .expect("An error occurred taking the first number!");

        println!("\nEnter the second number: ");
        let mut string_number_2 = String::new();
        io::stdin()
            .read_line(&mut string_number_2)
            .expect("An error occurred taking the second number!");

        let number_1: i64 = string_number_1.trim().parse().unwrap();
        let number_2: i64 = string_number_2.trim().parse().unwrap();

        if number_2 == 0 {
            println!("\nWe can't divide by zero :(\n");
            calculator();
        }

        let result = calculator_functions::operations::divide(number_1, number_2);
        println!();
        print!("Dividing {} / {} = {}", number_1, number_2, result);
        println!("\n\n");
        calculator();
    } else if user_option.trim() == "5" {
        println!("Bye! :)");
        exit(0);
    } else {
        println!("\nOops! it seems that we don't have that option :(\n");
        calculator();
    }
}
