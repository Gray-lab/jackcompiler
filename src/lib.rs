use std::{path::{PathBuf, Path}, fs};

#[derive(Debug)]
pub struct Config {
    pub file_paths: Vec<PathBuf>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("incorrect number of arguments");
        };
        
        let path = Path::new(&args[1]);
        let mut file_paths: Vec<PathBuf> = Vec::new(); 

        let metadata = match fs::metadata(path) {
            Ok(md) => md,
            // What if I want to return the actual error?
            Err(_) => {
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
            // If directory, add all .jack files to config
            let paths = match fs::read_dir(path) {
                Ok(paths) => paths,
                Err(_) => return Err("unable to access directory")
            };

            // Using unwrap here instead of handling the errors
            file_paths = paths
                .map(|path| path.unwrap().path())
                .filter(|path| { path.is_file() && path.extension().unwrap() == "jack" })
                .collect();
        }

        Ok(Config{ file_paths })
        
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn read_file_jack() {
        // requires directory 'test_dir' to contain file 'test1.jack'
        let mut args = Vec::new();
        args.push(String::from("arg1"));
        args.push(String::from("./test_dir/test1.jack"));

        let result_config = Config::build(&args).unwrap();

        let filepath = PathBuf::from_str(&args[1]).unwrap();

        assert!(result_config.file_paths.contains(&filepath));
    }

    #[test]
    fn read_directory_with_jack_files() {
        // requires directory 'test_dir' to contain files 'test1.jack' & 'test2.jack'
        let mut args = Vec::new();
        args.push(String::from("arg1"));
        args.push(String::from("./test_dir"));

        let result_config = Config::build(&args).unwrap();

        // both these .jack file paths should be saved in the config
        let filepath1 = PathBuf::from_str("./test_dir/test1.jack").unwrap();
        let filepath2 = PathBuf::from_str("./test_dir/test2.jack").unwrap();
        
        // this .txt file path should not be saved
        let badfilepath = PathBuf::from_str("./test_dir/test3.txt").unwrap();

        assert!(result_config.file_paths.contains(&filepath1));
        assert!(result_config.file_paths.contains(&filepath2));
        assert!(!result_config.file_paths.contains(&badfilepath));
    }
}
