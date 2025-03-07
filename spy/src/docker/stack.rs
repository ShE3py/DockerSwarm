use serde::Deserialize;
use std::ffi::OsStr;

#[derive(Debug, Deserialize)]
pub struct Task {
    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "ID")]
    pub id: String,
}

pub fn ps(stack: impl AsRef<OsStr>) -> Vec<Task> {
    let stdout = super::docker(["stack", "ps", "--format", "json"], stack);
    
    stdout.split(|b| *b == b'\n')
        .filter_map(|line| Some(line.trim_ascii()).filter(|line| !line.is_empty()))
        .filter_map(|line| match serde_json::from_slice::<Task>(line) {
            Ok(task) => Some(task),
            Err(e) => {
                eprintln!("error: failed to parse the following as a task: {e}");
                eprintln!("{}", String::from_utf8_lossy(line));
                None
            }
        })
        .collect()
}
