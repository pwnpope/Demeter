use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::io::Write;

// takes in a C file -> cleans white spaces, comments and tabs 
fn house_keeping(vector: Vec<String>) -> Vec<String> {
    let mut new_vec = Vec::new();
    let mut comment_flag = false;

    for element in vector.iter() {
        let trimmed_element = element.trim();

        if trimmed_element.starts_with("/*") { comment_flag = true; }

        if comment_flag && trimmed_element.ends_with("*/") {
            comment_flag = false;
            continue;
        }

        if trimmed_element.starts_with("//") { continue; }

        let mut code_element = trimmed_element.to_string();

        if code_element.contains("//") {
            if let Some(comment_start) = code_element.find("//") {
                code_element.replace_range(comment_start.., "");
            }
        }

        if comment_flag { continue; }

        if !code_element.is_empty() {
            let data = format!("{}", code_element);
            new_vec.push(data);
        }
    }
    new_vec
}


fn read_file(filepath: &str) -> io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {lines.push(line?);}
    Ok(lines)
}


// file: full path to file
fn reader(file: &str) -> io::Result<Vec<String>> {
    let data = read_file(file)?;
    let lines = house_keeping(data.clone());
    Ok(lines)
}


pub fn file_formatter(file_path: &str, verbose: bool) -> io::Result<()> {
    let lines_reformatted = reader(file_path);
    let mut file = File::create(file_path)?;
    for line in lines_reformatted? { writeln!(file, "{}", line).expect("[ERROR] something bad happened when reformatting"); }
    if verbose == true { println!("reformatted : {}", file_path); }
    Ok(())
}
