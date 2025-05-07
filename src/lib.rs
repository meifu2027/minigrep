use std::fs;
use std::error::Error;

fn run_one (file_path: String) -> () {
    let content = fs::read_to_string(file_path)
    .expect("文件读取异常 \n");
println!("content:\n{}", content);
}
pub fn run_two (file_path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    println!("content:\n{}", content);
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
    Config {query_str, file_path}

}

pub struct Config {
    pub query_str: String,
    pub file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query_str = args[1].clone();
        let file_path = args[2].clone();
        Config {query_str, file_path}
    }
    
    fn build_one(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments!!!")
        }
        let query_str = args[1].clone();
        let file_path = args[2].clone();
        Config {query_str, file_path}
    }
    pub fn build_two(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!!!");
        }
        let query_str = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {query_str, file_path})
    }
}