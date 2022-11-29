use std::fs;
use std::error::Error;
use std::env;
pub struct Config{
    pub query:String,
    pub file_path:String,
    pub icase:bool
}
impl Config {

    pub fn build(mut  args:impl Iterator<Item=String> )->Result<Self,&'static str>{
        args.next();
        let query=match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a query string.")
        };
        let file_path=match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a file path.")
        };
        let icase=env::var("IGNORE_CASE").is_ok();
        Ok(
            Self { query, file_path, icase}
        )
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_build(){
        let args=["test".to_string(),"test".to_string()];
        let q=Config::build(args.into_iter());
        assert!(q.is_err());
    }
    #[test]
    fn test_config(){
        let args=["exe".to_string(),"test".to_string(),"test".to_string()];
        let q=Config::build(args.clone().into_iter()).unwrap();
        assert_eq! (q.file_path,args[2]);
        assert_eq!(q.query,args[1]);
    }
    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
pub fn run (config:Config)->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.file_path)?;
    
    if config.icase {
        for line in search_case_insensitive(&config.query, &contents){
            println!("{line}");
        }
    }else
    {
        for line in search(&config.query, &contents){
        println!("{line}");
        }
    }
    Ok(())
}
pub fn search<'a> (qry:&str,contents:&'a str)->Vec<&'a str>{
    contents.lines().filter(|line| line.contains(qry)).collect()
}
pub fn search_case_insensitive<'a>(qry:&str,contents:&'a str)->Vec<&'a str>{
    let qry=qry.to_lowercase();
    contents.lines()
    .filter(|line|{line.to_lowercase().contains(&qry)}).collect()
}