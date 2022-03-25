// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_number: Option<i32>,
}

fn main() {
    let student = Student {
        name: "Ian".to_owned(),
        locker_number: Some(15),
    };

    println!("Student: {}", student.name);
    match student.locker_number {
        Some(num) => println!("Locker number: {}", num),
        None => println!("No locker assigned."),
    }
}
