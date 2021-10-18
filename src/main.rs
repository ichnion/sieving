use structopt::StructOpt;
use std::fs::File;
use std::io::{self, Write};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;
use colored::*;

mod regex_functions;
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
    let regex_list=vec![regex_functions::create_iban(),regex_functions::create_email_address(),regex_functions::create_ip_address(),
    regex_functions::create_japanese_phone_number1(),regex_functions::create_japanese_phone_number2(),regex_functions::create_japanese_phone_number3()];
    let regex_list_names=vec!["IBAN","EMAIL ADDRESS","IP ADDRESS","JAPANESE PHONE NUMBER","JAPANESE PHONE NUMBER","JAPANESE PHONE NUMBER"];
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
                    println!("{}",list[0]);
                }
                else if list.len()>1 {
                    println!("There are {} potential {} in this file: {}, do you want to hide them?\n{}\n{}/{}:",list.len().to_string().red().bold(),regex_list_names[j].red().bold(),d_name.red().bold(),"Be careful, this will change irredemiably your file.".yellow(),"YES".green(),"NO".red());
                    i=1;
                    count_data=1;
                    for elements in 0..list.len() {
                        println!("{}",list[elements]);
                    }
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

