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
    println!("Hello, world!");
}
