use std::{env, io::Write};
mod file_utils;
use chrono::{Datelike, Utc};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let tablename = &args[2];
    let file_vectors = file_utils::deblock_and_remove_rdw(filename);
    //TODO implement a filter that builds the file that I want to generate
    let mut table_sub_indicator = String::from("A");

    let now = Utc::now();
    let new_file_name = format!(
        "{}_{}-{:02}-{:02}.txt",
        tablename,
        now.year(),
        now.month(),
        now.day(),
    );
    let mut new_file = File::create(new_file_name).unwrap();

    for f in file_vectors {
        // this minimal lenth is related to the checks we must do before performing a slice extraction
        if f.len() > 7 {
            // Since the header (IP0000T1) is the first thing to be read, and it contains the other table names
            // eg: IP0040T1, it will set the table_sub_indicator first and then it will be searched in all the file
            if &f[11..14] == format!("IP0") && &f[11..27] == format!("IP0000T1{}", tablename) {
                let s = f.as_str();
                table_sub_indicator.push_str(&s[243..246]);
            }

            // having len > 1, means it was filled by the if that is up here
            if table_sub_indicator.len() == 4 && &f[7..11] == &table_sub_indicator {
                println!("{}", f);
                writeln!(&mut new_file, "{}", f).unwrap();
            }
        }
    }
    Ok(())
}
