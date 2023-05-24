use crate::task::Task;

pub fn read_csv_file(path: &str) -> Vec<Task> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut tasks: Vec<Task> = Vec::new();

    for taskResult in rdr.records() {
        let task: Task = taskResult.unwrap();
        tasks.push(task);
    }

    tasks
}

pub fn write_csv_file(path: &str, tasks: &Vec<Task>) {
    let mut wtr = csv::Writer::from_path(path).unwrap();
    for task in tasks {
        wtr.serialize(task).unwrap();
    }
}
