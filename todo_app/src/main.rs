use std::fs::OpenOptions;
use std::io::{self, Write, BufRead, BufReader};


#[derive(Debug)]
struct Task {
    description:String,
    completed:bool,
}
fn main() {
    let mut tasks:Vec<Task> = Vec::new();


    // Load existing tasks from file
    if let Ok(file)  = OpenOptions::new().read(true).open("todo.txt"){
        let reader = BufReader::new(file);

        for line in reader.lines() {

            if let Ok(line) = line {
                // Format: 0|Buy milk
                let parts: Vec<&str> = line.splitn(2, '|').collect();
                if parts.len() == 2 {
                    let completed = parts[0] == "1";
                    tasks.push(Task{
                        completed,
                        description: parts[1].to_string()
                    });
                }
            }
        }
    }

    loop{
        println!("\n-- Menu --");
        println!("1. Add a task");
        println!("2. Mark task as done");
        println!("3. Show all tasks");
        println!("4. Exit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).unwrap();

        let choice = choice.trim();
        match choice {
            "1" => {
                println!("Enter task:");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                let task = Task {
                    description: desc.trim().to_string(),
                    completed:false
                };
                tasks.push(task);
                save_tasks(&tasks);

            }
            "2" => {
                println!("\n Which task you want to mark as done? ");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}. [{}] {}", i + 1, if task.completed { "x" } else { " " }, task.description);
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    if let Some(task) = tasks.get_mut(index - 1) {
                        task.completed = true;
                        save_tasks(&tasks);
                        println!("Marked as done!");

                    }else {
                        println!("Invalid task number.");
                    }
                }
            }
            "3" => {
                println!("\n Tasks:");
                for(i,task) in tasks.iter().enumerate() {
                    println!("{}. [{}] {}", i + 1, if task.completed { "x" } else { " " }, task.description);
                }
            }
            "4" => break,
            _ =>  println!("Invalid choice!"),

            
        }
    
       
    }

   
}


fn save_tasks(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // overwrite file
        .create(true)
        .open("todo.txt")
        .expect("Cannot open file");

    for task in tasks {
        let line = format!("{}|{}\n", if task.completed { "1" } else { "0" }, task.description);
        file.write_all(line.as_bytes()).unwrap();
    }
}