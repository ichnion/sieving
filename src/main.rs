use structopt::StructOpt;
use std::fs::File;
use std::io::{self, Write};
use regex::Regex;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;
use colored::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), io::Error> {
    let args = Cli::from_args();
    let mut i;
    let mut count_data=0;
    let total_count = WalkDir::new(&args.path).into_iter().count();
    let pb = ProgressBar::new(total_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.purple} [{elapsed_precise}] [{bar:70.cyan/blue}] ({pos}/{len}, ETA \
                 {eta})",
            )
            .progress_chars("#>-"),
    );
    for i in 0..=total_count {
        pb.set_position(i as u64);
        thread::sleep(Duration::from_millis(124));
    }
    let regex_list=vec![create_iban(),create_email_address(),create_ip_address()];
    let regex_list_names=vec!["IBAN","EMAIL ADDRESS","IP ADDRESS"];
    for j in 0..regex_list.len() {
        let re = &regex_list[j];
        for entry in WalkDir::new(&args.path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let d_name = entry.path().to_string_lossy();
            if !entry.path().is_dir() {
                let mut list = Vec::new();
                let content = std::fs::read_to_string(entry.path()).expect("could not read file");
                for caps in re.captures_iter(&content) {
                    let text1 = caps.get(0).map_or("", |m| m.as_str());
                    list.push(text1);
                }
                if list.len()==1 {
                    println!("There is {} potential {} in this file: {}, do you want to hide it?\n{}\n{}/{}:", "1".red().bold(),regex_list_names[j].red().bold(), d_name.red().bold(),"Be careful, this will change irredemiably your file.".yellow(),"YES".green(),"NO".red());
                    i=1;
                    count_data=1;
                }
                else if list.len()>1 {
                    println!("There are {} potential {} in this file: {}, do you want to hide them?\n{}\n{}/{}:",list.len().to_string().red().bold(),regex_list_names[j].red().bold(),d_name.red().bold(),"Be careful, this will change irredemiably your file.".yellow(),"YES".green(),"NO".red());
                    i=1;
                    count_data=1;
                }
                else {
                    i=0;
                }
                if i==1 {
                    let mut response = String::new();
                    io::stdin().read_line(&mut response).expect("Failed to read line");
                    if response.trim().to_lowercase()=="yes" {
                        let data = re.replace_all(&content,"xxx");
                        let mut dst = File::create(entry.path())?;
                        dst.write(data.as_bytes())?;
                        println!("Changes saved");
                        println!("");
                    }
                    else if response.trim().to_lowercase()=="no" {
                        println!("No changes made\n");
                        println!("");
                    }
                    else {
                        println!("Invalid response.");
                        println!("");
                    }
                }
            }
        }
    }
    if count_data!=1 {
        println!("No sensitive data detected");
        println!("");
    }
    Ok(()) 
}

fn create_iban() -> regex::Regex {
    return Regex::new(r"(?m)[A-Z]{2}\d{25}\b").unwrap();
}

fn create_email_address() -> regex::Regex {
    return Regex::new(r"(?m)([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
}

fn create_ip_address() -> regex::Regex {
    return Regex::new(r"(?m)[0-9]{1,3}(\.[0-9]{1,3}){3}\b").unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iban_check() {
        let re=create_iban();
        assert!(re.is_match("FR7630001007941234567890185"));
    }
    #[test]
    fn email_check() {
        let re=create_email_address();
        assert!(re.is_match("gawen@georepublic.de"));
    }
    #[test]
    fn ip_check() {
        let re=create_ip_address();
        assert!(re.is_match("255.255.255.255"));
    }
}
