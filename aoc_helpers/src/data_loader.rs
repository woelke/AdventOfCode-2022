use std::fs::File;
use std::io::Read;
use std::path::Path;

fn get_lines(file: &str) -> Result<Vec<String>, String> {
    let mut res = vec![];
    let mut file_content = String::new();

    match File::open(Path::new(file)) {
        Err(why) => Err(why.to_string()),
        Ok(mut fd) => match fd.read_to_string(&mut file_content) {
            Err(why) => Err(why.to_string()),
            Ok(_) => {
                file_content
                    .lines()
                    .for_each(|line| res.push(line.to_string()));
                Ok(res)
            }
        },
    }
}

pub struct DataLoader {
    pub(crate) lines: Vec<String>,
}

impl DataLoader {
    pub fn from_file(file: &str) -> DataLoader {
        match get_lines(file) {
            Ok(lines) => DataLoader { lines },
            Err(msg) => panic!("Failed to read file {file}. Reason: {msg}"),
        }
    }

    pub fn from_data(data: &Vec<String>) -> DataLoader {
        let mut res: Vec<String> = Vec::new();
        for d in data.iter() {
            res.push(d.to_string());
        }
        DataLoader { lines: res }
    }

    pub fn test_result(&self) -> String {
        self.lines[0].to_string()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.lines.iter()
    }

    pub fn data(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.chars())
            .flatten()
            .collect()
    }
}
