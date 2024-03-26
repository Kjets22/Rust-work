use clap::{command,Arg,ArgMatches};
use url::Url;
use linkify::{LinkFinder, LinkKind};
use std::fs;
use reqwest::{get};


fn cli()->ArgMatches{
	let match_result=command!()
	.about("give a folder with url files, the program will return broken urls")
	.arg(
		Arg::new("folder")
		.short('f')
		.help("input a folder directory to get broken urls")
//		.value_parser(value_parser!(PathBuf))
	)
	.get_matches();
	match_result
}

fn main() {
	println!("working");
    let match_result=cli();
	if let Some(folder_directory) = match_result.get_one::<String>("folder") {
//		let folder_directory=match_result.get_one::<String>("folder");
		match std::fs::read_dir(folder_directory){
			Ok(folder_path)=>{
				for file_path in folder_path {
                    match fs::read_to_string(file_path.unwrap().path().into_os_string()){
                        Ok(contents)=>{
                            let finder= LinkFinder::new();
                            let links: Vec<_>=finder.links(&contents).collect();
                            for link in links{
                                let link=reqwest::get(link.as_str());
                                match link.status(){
                                    Ok(_l)=>{
                                        println!("{}",link);
                                        println!("running");
                                        continue;
                                    }
                                    Err(_e)=>{
                                        println!("{} broken link it works you not fucked",link);
                                    }
                                }                  
                                
                            }
                        }
                        Err(_e)=>{
                            continue;
                        }
				    }
				}
			}
			Err(_e)=>{
				println!("This folder directory is not valid");
			}
		}
	}
}
