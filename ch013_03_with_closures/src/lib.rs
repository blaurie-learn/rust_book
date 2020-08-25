use std::fs;
use std::error::Error;
use std::env;

//since we broke the main logic out to a lib, it is easier to write unit tests for the core of the
//code.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }

}


//the data used to return from the search will live as long as the data in contents since it's the
//same data
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    //iterator adaptors can rewrite this whole function
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

//Between the above and below, which should be preferred?
//Most rust programmers will prefer to use iterator adaptors for visuals and clarity,
//but what about performance?
//Truth is that the iterator adaptors should compile to be equivalently as fast as hard loops and
//(in this case, anyway) can even produce better, faster compiled code!

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }

    results
}



pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

//using a struct instead:
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {

    //Args implements Iterator, and we'll mutate it by iterating
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();    //the first arg is the path + program (pretty standard)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }

}
