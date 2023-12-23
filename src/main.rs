use std::io;

fn main() {
    println!("SIMPLE TODO LIST");
    println!("------------------");
    println!("Enter todos (Type 'exit' to finish):");

    let mut todo_list: Vec<String> = Vec::new();
    
    loop {
        let mut todo = String::new();
        
        io::stdin()
            .read_line(&mut todo)
            .expect("Failed to read line");
        
        todo = todo.trim().to_string();

        if todo.to_lowercase() == "exit" {
            break;
        }

        todo_list.push(todo.clone());

        println!("Added: {}", todo);
    }

    println!("Todo List:");
    for (index, value) in todo_list.iter().enumerate() {
        println!("{}: {}", index + 1, value);
    }
}
