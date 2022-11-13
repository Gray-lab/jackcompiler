use std::{ env, fs, path::Path, ffi::OsStr };
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).expect("hoping the config builds correctly");

    
}

struct Config {
    file_paths: Vec<Box<&Path>>,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("incorrect number of arguments");
        };
        let path = Path::new(&args[1]);
        let mut file_paths: Vec<Box<&Path>> = Vec::new(); 

        let metadata = match fs::metadata(path) {
            Ok(md) => md,
            // What if I want to return the actual error?
            Err(_) => return Err("problem accesing filepath metadata"),
        };

        // Check if the path is a file or a directory
        if metadata.is_file() {

            // If file, check that it is a .jack file and add to config
            if path.extension().and_then(OsStr::to_str) == Some(".jack") {
                file_paths.push(Box::new(path));
            } else {
                return Err("filename had incorrect extension")
            }

        } else {
            // If directory, add all .jack files to config
            let paths = fs::read_dir(path).expect("should be able to read directory");
            for path in paths {
                if path.expect("should have valid paths").extension().and_then(OsStr::to_str) == ".jack" {
                    file_paths.push(Box::new(path));
                }
            }

        }

        Ok(Config{ file_paths })
        
    }
}