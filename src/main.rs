use std::collections::HashMap;
use std::io;

///Add a new student with the following note to the map of students
fn add_student(notes: &mut HashMap<String, i32>, name: String, value: i32) {
    notes.insert(name, value);
}

///Delete a student from the map of students
fn remove_student(notes: &mut HashMap<String, i32>, name: String) {
    if notes.contains_key(&name) {
        notes.remove(&name);
    } else {
        println!("Student not found in the class.");
    }
}

///Get the student note for the given name
fn get_student(notes: &mut HashMap<String, i32>, name: String) -> Result<i32, String> {
    match notes.get(&name) {
        Some(&note) => Ok(note),
        None => Err("Bad Request : Student doesn't exist".to_string()), // Return -1 if the student is not found in the hashmap
    }
}

///Update the student note by the given value
fn update_student_note(notes: &mut HashMap<String, i32>, name: String, value: i32) {
    if notes.contains_key(&name) {
        notes.insert(name, value);
    } else {
        println!("Student not found in the class.");
    }
}

///Print all notes for all students
fn preview_class_notes(notes: &mut HashMap<String, i32>) {
    println!("Class Notes:");
    for (name, note) in notes.iter() {
        println!("{} have the following note : {}", name.trim(), note)
    }
}

///Return the mean of class notes
fn mean(notes: &HashMap<String, i32>) -> f32 {
    notes.values().sum::<i32>() as f32 / notes.len() as f32
}

fn main() {
    let mut class_notes: HashMap<String, i32> = HashMap::new();

    loop {
        print!("");
        println!("1. Add a student note");
        println!("2. Delete a student note");
        println!("3. Get student note");
        println!("4. Update student note");
        println!("5. Preview class notes");
        println!("6. Calculate class mean");
        println!("7. Exit");
        println!("Choose an option: ");
        let input = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input
        };

        match input.trim() {
            "1" => {
                println!("Enter the name of the student : ");
                let mut student_name = String::new();
                io::stdin()
                    .read_line(&mut student_name)
                    .expect("Failed to read name");
                println!("Enter the note : ");
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Failed to read");
                let i_value = value.trim().parse::<i32>().unwrap_or(-1);
                if i_value == -1 {
                    println!("Invalid note value. Please enter a valid number.");
                    continue;
                }
                add_student(&mut class_notes, student_name, i_value);
            }
            "2" => {
                println!("Enter the name of the student : ");
                let mut student_name = String::new();
                io::stdin()
                    .read_line(&mut student_name)
                    .expect("Failed to read name");
                remove_student(&mut class_notes, student_name);
            }
            "3" => {
                println!("Enter the name of the student : ");
                let mut student_name = String::new();
                io::stdin()
                    .read_line(&mut student_name)
                    .expect("Failed to read name");
                let note = get_student(&mut class_notes, student_name.clone());
                match note {
                    Ok(note) => println!("{} get the note : {}", &student_name.trim(), note),
                    Err(err) => println!("{}", err),
                }
            }
            "4" => {
                println!("Enter the name of the student : ");
                let mut student_name = String::new();
                io::stdin()
                    .read_line(&mut student_name)
                    .expect("Failed to read name");
                println!("Enter the new note : ");
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Failed to read");
                let i_value = value.trim().parse::<i32>().unwrap_or(-1);
                if i_value == -1 {
                    println!("Invalid note value. Please enter a valid number.");
                    continue;
                }
                update_student_note(
                    &mut class_notes,
                    student_name,
                    value.trim().parse().expect("Failed to parse"),
                );
            }
            "5" => preview_class_notes(&mut class_notes),
            "6" => {
                let mean_note = mean(&class_notes);
                println!("The mean of class notes is : {}", mean_note);
            }

            "7" => break,
            _ => println!("Option invalide"),
        }
    }
}
