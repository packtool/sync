use glob::glob;
use std::fs::File;
use serde_json::Value;

use std::io::{ Write, Read};
use std::path::Path;
use clap::{Command, Arg};
use crate::lib::merge_jsons;
use crate::lib::{detect_differences, apply_differences};
mod lib; // Add this line to import the `lib` module
/*
     Read the package.json file and return the list of workspaces
 */
fn read_package_json(file: &str) -> Vec<String> {
    let mut file = File::open(file).expect("Failed to open package.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read package.json");

    let json: Value = serde_json::from_str(&contents).expect("Failed to parse package.json");
    let workspaces = match json["workspaces"].as_array() {
        Some(array) => array.iter().map(|workspace| workspace.as_str().unwrap_or("").to_string()).collect(),
        None => Vec::new(),
    };
    workspaces
}

/*
    Find and return the path of the package.json file in the specified folder
*/
fn find_package_json(folder: &str,package_name:&str) -> Vec<String> {
    let pattern = Path::new(folder).join(package_name).to_str().unwrap().to_string();
    let mut package_json_paths = Vec::new();

    if let Ok(entries) = glob(&pattern) {
        for entry in entries {
            if let Ok(path) = entry {
                if let Some(file_name) = path.file_name() {
                    if file_name == package_name {
                        package_json_paths.push(path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    package_json_paths
}

fn read_extends(file_path: &str) -> Vec<String> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    let extends = match json["packtool"]["extends"].as_array() {
        Some(array) => array.iter().map(|value| value.as_str().unwrap_or("").to_string()).collect(),
        None => Vec::new(),
    };
    extends
}

fn get_path(file_path: &str, relative_path: &str) -> String {
    let file_dir = Path::new(file_path).parent().unwrap();
    let joined_path = file_dir.join(relative_path);
    joined_path.to_str().unwrap().to_string()
}

fn create_file_if_not_exists(file_path: &str) {
    let path = Path::new(file_path);

    if !path.exists() {
        let mut file = File::create(path).expect("Failed to create file");
        file.write(b"{}").expect("Failed to write to file");
        println!("Created file: {}", file_path);
    }
}




fn main() {
    // argument parsing
    let matches = Command::new("MyApp").arg(
                Arg::new("mode")
                    .help("The mode")
                    .value_parser(["pull", "push"])
                    .required(true)
                    .index(1),
            )
            .get_matches();
    let mode : &String = matches.get_one("mode").expect("default");
    // read workspaces and package.json files
    let workspaces = read_package_json("package.json"); 
        for workspace in &workspaces {
            let package_json_paths = find_package_json(&workspace, "package.json");
            for package_json_path in package_json_paths {
                let package_json_contents_str = std::fs::read_to_string(&package_json_path).expect("Failed to read extend file");
                // get the base file
                let base_path = get_path(&package_json_path, "package.base.json");
                create_file_if_not_exists(&base_path);
                let mut package_base_json_file = File::open(&base_path).expect("Failed to open extend file");
                let mut package_base_json_contents = String::new();
                package_base_json_file.read_to_string(&mut package_base_json_contents).expect("Failed to read extend file");
                let mut package_base_json_contents_str: &str = &package_base_json_contents;
                // let package_base_json_value : Value = serde_json::from_str(package_base_json_contents_str).expect("Failed to parse package.json");
                // read the extends
                let extends = read_extends(&base_path);
                let _new_json = &mut package_base_json_contents_str;
                let mut current_json = package_base_json_contents.clone();
                for extend in extends.iter().rev() {
                    let extend_path = get_path(&package_json_path, &extend);
                    create_file_if_not_exists(&extend_path);
                    let extend_contents = std::fs::read_to_string(&extend_path)
                    .expect("Failed to read extend file");
                    current_json = merge_jsons(&current_json, &extend_contents);
                }

                if mode == "pull" {
                    let differences = detect_differences(&current_json,&package_json_contents_str );
                    let mut package_base_json_value : Value = serde_json::from_str(&package_base_json_contents).expect("Failed to parse package.json");
                    apply_differences(  &mut package_base_json_value, &differences);
                    let result2 =serde_json::to_string_pretty(&package_base_json_value).unwrap();
                    // open the file and write the new content
                    let mut file = File::create(&base_path).expect("Failed to create file");
                    file.write_all(result2.as_bytes()).expect("Failed to write to file");
                } else {// update the package.json
                    let mut file = File::create(&package_json_path).expect("Failed to create file");
                    file.write_all(current_json.as_bytes()).expect("Failed to write to file");
                }
                let differences = detect_differences(&current_json,&package_json_contents_str );
                let mut package_base_json_value : Value = serde_json::from_str(&package_base_json_contents).expect("Failed to parse package.json");
                apply_differences(  &mut package_base_json_value, &differences);
                let _result2 =serde_json::to_string_pretty(&package_base_json_value).unwrap();
            }
        }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_package_json() {
        // Create a temporary package.json file for testing
        let package_json = r#"{
            "workspaces": [
                "workspace1",
                "workspace2",
                "workspace3"
            ]
        }"#;
        let mut file = NamedTempFile::new().unwrap();
        file.write(package_json.as_bytes()).unwrap();

        // Call the function and assert the result
        let workspaces = read_package_json(file.path().to_str().unwrap());
        assert_eq!(workspaces, vec!["workspace1", "workspace2", "workspace3"]);
    }
    #[test]
    fn test_empty_package_json() {
        // Create a temporary package.json file for testing
        let package_json = r#"{
            "version": 5
        }"#;
        let mut file = NamedTempFile::new().unwrap();
        file.write(package_json.as_bytes()).unwrap();

        // Call the function and assert the result
        let workspaces = read_package_json(file.path().to_str().unwrap());
        assert_eq!(workspaces, Vec::<String>::new());
    }
}
