
use std::env;
use minigrep::Config;
use std::process;

fn main() {
    let args: Vec<String> =  env::args().collect();
    // dbg!(args);
    // let query_str = args[1].clone();
    // let file_path = args[2].clone();
    // let (query_str, file_path) = extract_params(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    // let config = Config::build_one(&args);
    let config = Config::build_two(&args).unwrap_or_else(|err| {
        println!("error msg: {}", err);
        process::exit(1)
    });
    println!("query_str: {}\nfile_path: {} \n",config.query_str, config.file_path);

    // run_one(config.file_path);
    // run_two(config.file_path).unwrap_or_else(|err| {
    //     println!("错误信息:\n{err}");
    //     process::exit(1)
    // });
    if let Err(err) = minigrep::run_two(config.file_path) {
        println!("错误信息:\n{err}");
        process::exit(1)
    };
    // test commit
}
