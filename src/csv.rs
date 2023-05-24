use crate::task::Task;
use std::io::Error;

pub fn read_csv_file(path: &str) -> Result<Vec<Task>, Error> {
    let rdr = csv::Reader::from_path(path);
    let mut tasks: Vec<Task> = Vec::new();

    for task_result in rdr?.deserialize() {
        let task: Task = task_result?;
        tasks.push(task);
    }

    Ok(tasks)
}

pub fn write_csv_file(path: &str, task: &Task) {
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.serialize(task).unwrap();
}
