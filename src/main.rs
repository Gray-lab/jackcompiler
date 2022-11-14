use std::{ env, fs, path::{Path, PathBuf} };
fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let config = Config::build(&args).expect("hoping the config builds correctly");

    dbg!(config);

}

#[derive(Debug)]
struct Config {
    file_paths: Vec<PathBuf>,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("incorrect number of arguments");
        };
        
        let path = Path::new(&args[1]);
        let mut file_paths: Vec<PathBuf> = Vec::new(); 

        let metadata = match fs::metadata(path) {
            Ok(md) => md,
            // What if I want to return the actual error?
            Err(error) => {
                dbg!(error);
                return Err("problem accesing filepath metadata")
            },
        };

        // Check if the path is a file or a directory
        if metadata.is_file() {

            let ext = match path.extension() {
                Some(ext) => ext,
                None => return Err("unable to access file extension")
            };

            // If file, check that it is a .jack file and add to config
            if ext == "jack" {
                file_paths.push(path.to_path_buf());
            } else {
                return Err("filename had incorrect extension")
            }

        } else {
            // println!("Path is a directory!");

            // If directory, add all .jack files to config
            let paths = match fs::read_dir(path) {
                Ok(paths) => paths,
                Err(_) => return Err("unable to access directory")
            };

            // I need to access the extension of each path 
            // how to do that in a functional paradigm?
            file_paths = paths
                .map(|path| path.unwrap().path())
                .filter(|path| { path.is_file() && path.extension().unwrap() == "jack" })
                .collect();
        }

        Ok(Config{ file_paths })
        
    }
}