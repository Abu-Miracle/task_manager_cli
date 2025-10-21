use std::{error::Error, fmt::Display};

use crate::Task;

pub fn save_tasks<'a, T>(tasks: &[Task<T>], mut filename: &'a str) -> String
where
    T: Display,
{
    let mut string = String::new();
    for items in tasks {
        let file = format!(
            "{}|{}|{}|{:?}|{}\n",
            items.id, items.description, items.priority, items.status, items.metadata
        );
        string.push_str(&file);
    }

    string
}

// fn load_tasks(filename: &'a str) -> Result<Vec<_>, Error> {

// }
