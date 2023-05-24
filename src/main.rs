use crate::csv::{read_csv_file, write_csv_file};
use crate::task::{Task, TaskBuilder};
use std::io::Error;

pub mod csv;
pub mod json;
pub mod task;

pub const DESCRIPTION_PARAM: &str = "-d";
pub const TITLE_PARAM: &str = "-t";
pub const NEW_TASTE_PARAM: &str = "-n";
pub const UPDATE_TASK_PARAM: &str = "-u";
pub const SET_STATUS_PARAM: &str = "-s";
pub const DELETE_TASK_PARAM: &str = "-r";

fn main() {
    let args: Vec<String> = std::env::args().collect();
}

fn read_csv(path: &str) -> Result<Vec<Task>, Error> {
    for task in read_csv_file(path)? {
        println!("{:?}", task);
    }

    Ok(read_csv_file(path)?)
}

fn create_task(title: &str, description: &str) {
    let task = TaskBuilder::new()
        .with_title(title.to_string())
        .with_description(description.to_string())
        .build();

    write_csv_file("tasks.csv", &task);

    println!("{:?}", task);
}
