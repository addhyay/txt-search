use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    fn new(query: String, path: String, ignore_case: bool) -> Self {
        Self {
            query,
            path,
            ignore_case,
        }
    }
}

pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() == 1 {
        return Err("Two more arguments are required: \'search_item\' and \'file path\'");
    } else if args.len() < 3 {
        return Err("One more argument is required: \'file path\'");
    } else if args.len() > 4 {
        return Err("Only four arguments are expected.");
    }
    let query = args[1].clone();
    let path = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Ok(Config::new(query, path, ignore_case))
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;
    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    if res.is_empty() {
        println!("Not Found!");
    } else {
        for line in res {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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

    #[test]
    fn case_insensitive_search() {
        let query = "rUst";
        let contents = "\
            Rust:
            safe fast productive
            No trust available.";
        assert_eq!(
            vec!["Rust:", "No trust available."],
            search_case_insensitive(query, contents)
        );
    }
}
