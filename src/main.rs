#![allow(unused)]

mod cli;
mod storage;

use crate::cli::get_input;
use crate::storage::save_tasks;
use colored::Colorize;
use comfy_table::Table;

use std::{
    fmt::{Debug, Display},
    io, option,
};
fn main() {
    let task_1 = Task {
        id: 1,
        description: String::from("Buy Groceries"),
        priority: 2,
        status: TaskStatus::InProgress,
        metadata: String::from("Personal"),
    };

    let task_2 = Task {
        id: 2,
        description: String::from("Update Files"),
        priority: 2,
        status: TaskStatus::InProgress,
        metadata: String::from("Work"),
    };

    let task_3 = Task {
        id: 3,
        description: String::from("Read Book"),
        priority: 0,
        status: TaskStatus::InProgress,
        metadata: String::from("School"),
    };

    // let mut tasks = vec![&task_1, &task_2, &task_3];

    // fn check_priority(tasks: Vec<&Task<String>>) {
    //     for items in tasks {
    //         if items.priority > 3 || items.priority < 1 {
    //             panic!();
    //         }
    //     }
    // }

    // println!("Task 1: {:?}", task_1);
    // check_priority(tasks);

    // let task_4 = create_task(
    //     4,
    //     "Write Rust Code",
    //     1,
    //     String::from("Learning"),
    // );
    // println!("Task 4: {:?}", task_4);

    // let task_5 = create_task(
    //     5,
    //     "Write Cairo Code",
    //     2,
    //     String::from("Learning"),
    // );
    let mut all_tasks = TaskManager::new();
    // let mut tasks = all_tasks.tasks;

    // for items in &tasks {
    //     println!("{:?}", items);
    // }

    let category = Category {
        name: String::from("Work"),
    };

    // let task_a = create_task(5, "Write Cairo Code", 2, category);

    // list_tasks(&tasks);

    loop {
        println!(
            "\n{} {}", 
            "==== WELCOME TO TASK MANAGER CLI ====".green().bold(),
            "\n\n1) Add Task \n2) List Tasks \n3) Update Status \n4) Menu \n5) Print task table \n6) Get Task by ID \n7) Delete Task by ID \n8) Exit".italic(),
        );

        let option = get_input("\nEnter Option: ".dimmed().bold());

        match string_to_u32(option) {
            Ok(1) => {
                let description = get_input("Enter Description: ".dimmed().bold());
                let priority = get_input("Enter Priority: ".dimmed().bold());
                let metadata = get_input("Enter Metadata: ".dimmed().bold());

                match all_tasks.add_task(
                    description.as_str(),
                    string_to_u8(priority).unwrap(),
                    metadata,
                ) {
                    Ok(_) => println!(
                        "\nTask '{}' added successfully",
                        description.as_str().green()
                    ),
                    Err(e) => println!("\n{}", e.red()),
                }
            }

            Ok(2) => all_tasks.list_tasks(),
            Ok(3) => {
                all_tasks.list_tasks();
                let index = get_input("Enter Task Index: ".dimmed().bold());
                let task_id = string_to_u32(index).unwrap();

                println!(
                    "{}{}{}{}",
                    "\nChoose new status ".dimmed().bold(),
                    "\n\t1) Todo".blue(),
                    "\n\t2) InProgress\n\t".yellow(),
                    "3) Done".bright_green()
                );

                let status = get_input("Enter status: ".dimmed().bold());

                if let Some(task) = all_tasks.tasks.iter_mut().find(|x| x.id == task_id) {
                    match string_to_u32(status) {
                        Ok(1) => task.status = TaskStatus::Todo,
                        Ok(2) => task.status = TaskStatus::InProgress,
                        Ok(3) => task.status = TaskStatus::Done,
                        Ok(_) => println!("{}", "Priority must be between 1 and 3".red()),
                        Err(e) => println!("\nError: {}", e),
                    }
                } else {
                    println!("Task with ID: '{}' cannot be found", task_id)
                }
            }
            Ok(4) => {
                println!("\nSelect Status \n\t1) Todo\n\t2) InProgress\n\t3) Done");

                let status = get_input("Enter status: ".dimmed().bold());

                match string_to_u32(status) {
                    Ok(1) => {
                        let filtered: Vec<&Task<String>> = all_tasks
                            .tasks
                            .iter()
                            .filter(|x| x.status == TaskStatus::Todo)
                            .collect();

                        for items in filtered {
                            println!(
                                "{}. {} (Priority: {}, Status: {:?}, Metadata: {:?})",
                                items.id,
                                items.description,
                                items.priority,
                                items.status,
                                items.metadata
                            )
                        }
                    }
                    Ok(2) => {
                        let filtered: Vec<&Task<String>> = all_tasks
                            .tasks
                            .iter()
                            .filter(|x| x.status == TaskStatus::InProgress)
                            .collect();

                        for items in filtered {
                            println!(
                                "{}. {} (Priority: {}, Status: {:?}, Metadata: {:?})",
                                items.id,
                                items.description,
                                items.priority,
                                items.status,
                                items.metadata
                            )
                        }
                    }
                    Ok(3) => {
                        let filtered: Vec<&Task<String>> = all_tasks
                            .tasks
                            .iter()
                            .filter(|x| x.status == TaskStatus::Done)
                            .collect();

                        for items in filtered {
                            println!(
                                "{}. {} (Priority: {}, Status: {:?}, Metadata: {:?})",
                                items.id,
                                items.description,
                                items.priority,
                                items.status,
                                items.metadata
                            )
                        }
                    }
                    Ok(_) => println!("Invalid input"),
                    Err(e) => println!("\nError: {}", e),
                }
            }
            Ok(5) => all_tasks.print_table(),
            Ok(6) => {
                let task = get_input("Enter ID: ".dimmed().bold());
                let task_u32 = string_to_u32(task).unwrap();

                match all_tasks.tasks.iter().find(|x| x.id == task_u32) {
                    Some(task) => println!("{}", task.display()),
                    None => println!(
                        "{} {} {}",
                        "Task with ID".red(),
                        format!("'{task_u32}'").red(),
                        "cannot be found".red()
                    ),
                }
            }
            Ok(7) => {
                let task = get_input("Enter ID: ".black().bold());
                let task_u32 = string_to_u32(task).unwrap();

                match all_tasks.tasks.iter().find(|x| x.id == task_u32) {
                    Some(task) => {
                        let task_id = task.id;
                        let task_desc = task.description.clone();
                        all_tasks.delete_task_by_id(task_id);
                        println!("Successfully deleted '{}'", task_desc.red());
                    }
                    None => println!(
                        "{} {} {}",
                        "Task with ID".red(),
                        format!("'{task_u32}'").red(),
                        "cannot be found".red()
                    ),
                }
            }
            Ok(8) => break,
            Ok(_) => println!("Invalid Option"),
            Err(e) => println!("\nError: {}", e),
        }
    }
}

// Error handling
// Get Specific task
// Delete

#[derive(Debug)]
pub struct Task<T: Display> {
    pub id: u32,
    pub description: String,
    pub priority: u8,
    pub status: TaskStatus,
    pub metadata: T,
}

#[derive(Debug)]
struct TaskManager<T: Display> {
    tasks: Vec<Task<T>>,
}

#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    Done,
    InProgress,
    Todo,
}

#[derive(Debug)]
struct Category {
    name: String,
}

trait Displayable<T> {
    fn display(&self) -> String;
}

impl<T> Displayable<T> for Task<T>
where
    T: Display,
{
    fn display(&self) -> String {
        format!(
            "ID: {}, Description: {}, Priority: {}, Status: {:?}, Metadata: {}",
            self.id, self.description, self.priority, self.status, self.metadata
        )
    }
}

impl<T: Display> TaskManager<T> {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn add_task(&mut self, description: &str, priority: u8, metadata: T) -> Result<(), &str> {
        match priority {
            1..=3 => {
                let task = Task {
                    id: self.tasks.len() as u32 + 1,
                    description: description.to_string(),
                    priority,
                    metadata,
                    status: TaskStatus::Todo,
                };
                self.tasks.push(task);
                Ok(())
            }
            _ => Err("Error: Priority must be between 1 and 3"),
        }
    }

    fn get_task_by_id(&self, id: u32) -> Result<&Task<T>, String> {
        self.tasks
            .iter()
            .find(|x| x.id == id)
            .ok_or_else(|| "Task cannot be found".to_string())
    }

    fn delete_task_by_id(&mut self, id: u32) {
        self.tasks.retain(|x| x.id != id);
    }

    fn list_tasks(&self) {
        for items in &self.tasks {
            println!("{}", items.display());
        }
    }

    fn print_table(&self) {
        let mut table = Table::new();

        table.set_header(vec!["ID", "Description", "Priority", "Status", "Metadata"]);

        for items in &self.tasks {
            table.add_row(vec![
                items.id.to_string(),
                items.description.clone(),
                items.priority.to_string(),
                format!("{:?}", items.status),
                items.metadata.to_string(),
            ]);
        }

        println!("{table}");
    }
}

fn string_to_u32(string: String) -> Result<u32, String> {
    string
        .trim()
        .parse()
        .map_err(|_| "Invaiid Input".to_string())
}

fn string_to_u8(string: String) -> Result<u8, String> {
    string
        .trim()
        .parse()
        .map_err(|_| "Invaiid Input".to_string())
}

fn list_tasks<T: Debug + Display>(tasks: &Vec<Task<T>>) {
    for items in tasks {
        println!("{}", items.display());
    }
}
