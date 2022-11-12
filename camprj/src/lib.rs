
use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;
use serde::{Serialize,Deserialize};
use std::io::{BufReader};
use std::fs::File;
/// Goals of this library:
/// 
/// 1. Offer a way of reading the our project and machine config files and merging them together
/// 2. Offer a common api for working with our project files
/// 3. Offer a collection of functions to work with our project file
/// 
/// General Structure:
/// we are going to use rusts serder api to provide a method of serialising and 
/// deserialising data to and from configuration files. serder will be used as it
/// should allow us work with different file types without having to write a parser
/// for every single one of them. This being said YAML files should be considered
/// first class citizans as they are simple, readable and contain minimal uneeded
/// character.
/// 
/// This configuration file will be defined using the derived implentation given 
/// us be the preprocessor to reduce undeeded code. The goal of this struct is
/// allow the users to write this file with the minimal number options and 
/// therefore will end up with alot of optionals this is going to be a
/// programming nightmare with 1million match statements but whatever.
/// 
/// Features:
/// 
/// 1: a ci/cd interface allowing users to define a series of commands in the file 
///    as well as the order in which they will be preformed in to allow for
///    users to run a command such as "camprj build" and for it to clean the
///    project, call a package manager to ensure all dependancies are present,
///    and then call a build tool e.g. make to compile the application  
/// 2: an enviroment defintion system the allows the user to define the 
///    enviroment variables that will be set when the custom commands are run
///    this will also include a check for the files 
///    ~/.config/cam_prj/<project_name>.yaml and ~/.config/cam_prj/common.yaml
///    to allow for the overiding of particular values when runing on a particular
///    machine with the priorty order being with values lower on the list
///    allways being preserved over higher values so 1 will allway superseed 2:
///         1. ~/.config/cam_prj/<project_name>.yaml
///         2. ~/.config/cam_prj/common.yaml
///         3. .cam_prj.yaml

/// This is the top level file object that contains different sections used 
/// inside the configuration files
#[derive(Serialize, Deserialize,PartialEq,Debug)]
pub struct cam_prj {
    project: project,
    commands: targets
}

/// This is the project section containing meta info such as version 
#[derive(Serialize, Deserialize,PartialEq,Debug)]
pub struct project {
    name:    String,
    version: Option<String>,
    license: Option<String>,
}
/// This is the targets section 
#[derive(Serialize, Deserialize,PartialEq,Debug)]
pub struct targets {
    commands: Option<HashMap<String,String>>,
    enviroment: Option<HashMap<String,String>>
}

pub enum PrjError {
    NoSuchFile(String),
    UnknownFileError(String),
    ParsingFailure(String),
    UnkownFileType(String)
}


/// This is a helper method around serde to allow for the file to be read in
/// from vaiours different file types while allowing the consumers api to 
/// not care what that file type is
pub fn read_from(file: &String) -> Result<cam_prj,PrjError> {
    let extionsion: &str;
    match Path::new(&file).extension().and_then(OsStr::to_str) {
        Some(data) => {
            extionsion = data
        }
        None => {
            log::error!("Failed to get the extension for the file: {}! does the file exist?",file);
            return Err(PrjError::NoSuchFile(file.clone()));
        }
    };
    let reader: BufReader<File>;
    match File::open(&file) {
        Ok(dat) => {
            reader = BufReader::new(dat);
        }
        Err(err) => {
            log::error!("Failed to open file {} because {}",file, err);
            return Err(PrjError::UnknownFileError(format!("{}",err)));
        }
    }
    match extionsion {
        "yaml" => {
            match serde_yaml::from_reader(reader) {
                Ok(result) => {
                    return Ok(result);
                }
                Err(err) => {
                    log::error!("Failed to parse the yaml file: {}",err);
                    return Err(PrjError::ParsingFailure(format!("{}",err)));
                }
            }
        }
        _ => {
            log::error!("Config file type unsupported please consult the docs for valid file types");
            Err(PrjError::UnkownFileType(file.clone()))
        }
    } 
}
