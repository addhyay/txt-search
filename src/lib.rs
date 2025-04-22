use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    fn new(query: String, path: String) -> Self {
        Self { query, path }
    }
}

pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() == 2 {
        return Err("One more argument is required: \'file path\'");
    } else if args.len() < 3 {
        return Err("Two more arguments are required: \'query\' and \'file path\'");
    } else if args.len() > 3 {
        return Err("Only three arguments are expected.");
    }
    let query = args[1].clone();
    let path = args[2].clone();
    Ok(Config::new(query, path))
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;
    let res = search(&config.query, &contents);

    if res.is_empty() {
        println!("No matches found.");
    } else {
        for line in res {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_correct_line() {
        let query = "duct";
        let contents = "\
            Rust:
            safe fast productive
            Not available.";
        assert_eq!(vec!["safe fast productive"], search(query, contents));
    }
}
