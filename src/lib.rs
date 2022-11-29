use std::fs;
use std::error::Error;
use std::env;
pub struct Config<'a>{
    pub query:&'a String,
    pub file_path:&'a String,
    pub icase:bool
}
impl<'a> Config<'a> {
    pub fn new (args:&'a [String],icase:bool)->Self{
        if args.len()<3{
            panic!("not enough arguments");
        }
        Self { query: &args[1], file_path: &args[2], icase }
    }
    pub fn build(args:&'a [String])->Result<Self,&'static str>{
        if args.len()<3{
            Err("not enough arguments")
        }else{
            let icase=env::var("IGNORE_CASE").is_ok();
            Ok(Self::new(args,icase))
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_build(){
        let args=["test".to_string(),"test".to_string()];
        let q=Config::build(&args);
        assert!(q.is_err());
    }
    #[test]
    fn test_config(){
        let args=["exe".to_string(),"test".to_string(),"test".to_string()];
        let q=Config::build(&args).unwrap();
        assert_eq! (q.file_path,&args[2]);
        assert_eq!(q.query,&args[1]);
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