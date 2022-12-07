use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

// I wholeheartedly regret deciding to do AoC only in Rust.
// Due to time constraints I relied on a HashMap with the string containing the whole
// path to the directory as the key and the Directory struct instance as a value.
// I'm not proud of this but it works.

#[derive(Debug, Default)]
struct Directory {
    files: Vec<usize>,
    children_names: Vec<String>,
}

impl Directory {
    pub fn get_total_file_size(&self) -> usize {
        self.files.iter().sum()
    }
    pub fn add_file(&mut self, file_size: usize) {
        self.files.push(file_size);
    }
    pub fn add_directory(&mut self, new_directory_name: String) {
        self.children_names.push(new_directory_name);
    }
    pub fn contains_dir(&self, directory_name: String) -> bool {
        self.children_names.contains(&directory_name)
    }
}

fn vec_to_path(path_vector: &Vec<String>) -> String {
    if path_vector.len() == 0 {
        return "/".to_string();
    }
    return "/".to_string() + &path_vector.join("/") + "/";
}

fn calculate_size(directory_tree: &HashMap<String, Directory>, directory_path: String) -> usize {
    let current_directory_node = directory_tree.get(&directory_path).unwrap();
    let mut size: usize = 0;
    for child in current_directory_node.children_names.iter() {
        size += calculate_size(directory_tree, child.clone());
    }
    size += current_directory_node.get_total_file_size();
    return size;
}


fn get_directory_tree(input_filename: &str) -> Result<HashMap<String, Directory>, Box<dyn std::error::Error>> {
    let file = File::open(input_filename)
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    let root_path_vector = Vec::<String>::new();
    let root = Directory { ..Default::default() };

    let root_path = vec_to_path(&root_path_vector);
    let mut directory_tree: HashMap<String, Directory> = HashMap::new();

    directory_tree.insert(root_path.clone(), root);

    let mut current_directory_path_vector = Vec::<String>::new();
    let mut current_directory = root_path.clone();

    for raw_line in buf_reader.lines() {
        let mut line = raw_line?;
        if line == "" {
            continue;
        }
        line = line.trim().to_string();
        if line == "$ cd /" {
            current_directory_path_vector = Vec::new();
            current_directory = root_path.clone();
            continue;
        }
        let tokenized_line = &line.split(" ").collect::<Vec<&str>>()[..];
        if let [first_word, second_word] = tokenized_line {
            if first_word == &"dir" {
                let current_directory_node = directory_tree.get_mut(&current_directory).unwrap();
                if !current_directory_node.contains_dir(second_word.to_string()) {
                    let new_directory = Directory { ..Default::default() };

                    let mut new_dir_path_vec = current_directory_path_vector.clone();
                    new_dir_path_vec.push(second_word.to_string());
                    let new_dir_path = vec_to_path(&new_dir_path_vec);

                    current_directory_node.add_directory(new_dir_path.to_string());
                    directory_tree.insert(new_dir_path, new_directory);
                }
            }
            else if let Ok(file_size) = first_word.parse::<usize>() {
                let current_directory_node = directory_tree.get_mut(&current_directory).unwrap();
                current_directory_node.add_file(file_size);
            }
        }
        else if let [_dollar_sign, _cd_command, directory] = tokenized_line {
            if directory == &".." {
                if current_directory_path_vector.len() > 0 {
                    current_directory_path_vector.pop();
                    current_directory = vec_to_path(&current_directory_path_vector);
                }
            }
            else {
                current_directory_path_vector.push(directory.to_string());
                current_directory = vec_to_path(&current_directory_path_vector);
            }
        }
        else {
            panic!();
        }
    }
    return Ok(directory_tree);
}

pub fn run_first() -> Result<(), Box<dyn std::error::Error>> {
    let directory_tree = get_directory_tree("./inputs/day_7/first.txt")?;

    let mut result = 0;
    for directory_path in directory_tree.keys() {
        let total_dir_size = calculate_size(&directory_tree, directory_path.clone());
        if total_dir_size <= 100_000 {
            result += total_dir_size;
        }
    }
    println!("{:?}", result);
    Ok(())
}


pub fn run_second() -> Result<(), Box<dyn std::error::Error>> {
    let directory_tree = get_directory_tree("./inputs/day_7/second.txt")?;

    let root_path = "/".to_string();
    let total_disc_space = 70_000_000;
    let used_space = calculate_size(&directory_tree, root_path);
    let unused_space = total_disc_space - used_space;
    let space_needed = 30_000_000 - unused_space;
    let mut result = used_space;
    for directory_path in directory_tree.keys() {
        let total_dir_size = calculate_size(&directory_tree, directory_path.to_string());
        if total_dir_size <= result && total_dir_size >= space_needed {
            result = total_dir_size;
        }
    }
    println!("{:?}", result);
    Ok(())
}