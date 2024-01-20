struct Person {
    name: String,
    age: i16,
}

pub fn closures() {
    let add = |x, y| x + y;
    let result = add(5, 5);
    println!("The result is: {result}");

    let mut p1 = Person {
        name: "Gerson".to_string(),
        age: 22,
    };

    let mut modifying_age = |new_age| p1.age = new_age;
    modifying_age(23);
    modifying_age(28);
    println!("The new age is: {}", p1.age);
    println!("The name is: {}", p1.name);
}
