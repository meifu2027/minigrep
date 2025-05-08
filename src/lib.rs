use std::{env, fs, result};
use std::error::Error;

fn run_one (file_path: String) -> () {
    let content = fs::read_to_string(file_path)
    .expect("文件读取异常 \n");
println!("content:\n{}", content);
}
pub fn run_two (config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    // println!("content:\n{}", content);
    // for line in search(&config.query_str, &content) {
    //     println!("匹配行：{line}")
    // }
    let result = if config.ignore_case {
        search_case_insensitive(&config.query_str, &content)
    } else {
        // search_case_sensitive(&config.query_str, &content)
        search_case_sensitive_two(&config.query_str, &content)

    };
    for line in result {
        println!("匹配行：{line}")
    }
    Ok(())
}

fn extract_params(args: &[String]) -> (&String, &String) {
    let query_str = &args[1];
    let file_path = &args[2];
    (query_str, file_path)
}

fn parse_config(args: &[String]) -> Config {
    let query_str = args[1].clone();
    let file_path = args[2].clone();
    Config {query_str, file_path, ignore_case: false}

}

pub struct Config {
    pub query_str: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query_str = args[1].clone();
        let file_path = args[2].clone();
        Config {query_str, file_path, ignore_case: false}
    }
    
    fn build_one(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments!!!")
        }
        let query_str = args[1].clone();
        let file_path = args[2].clone();
        Config {query_str, file_path, ignore_case: false}
    }
    pub fn build_two(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!!!");
        }
        let query_str = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {query_str, file_path, ignore_case: false})
    }
    pub fn build_three(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!!!");
        }
        let query_str = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(value) => value == "1", 
            Err(_) => match args.get(3) {
                Some(value) => value == "-i",
                None => false,
            },
        };
        Ok(Config {query_str, file_path, ignore_case})
    }
    pub fn build_four(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query_str = match args.next() {
            Some(value) => value,
            None => return Err("query_str not exist!") 
        };
        let file_path = match args.next() {
            Some(value) => value,
            None => return Err("file_path not exist!") 
        };
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(value) => value == "1", 
            Err(_) => match args.next() {
                Some(value) => value == "-i",
                None => false,
            },
        };
        Ok(Config {query_str, file_path, ignore_case})
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
pub fn search_case_sensitive_two<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Duct
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn two_result() {
        let query = "duct";
        let contents = "\
Rust:
Duct
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Duct", "safe, fast, productive."], search_case_insensitive(query, contents));
    }
}