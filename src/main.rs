use std::env;

mod file_utils;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_vectors = file_utils::deblock_and_remove_rdw(filename);
    //TODO implement a filter that builds the file that I want to generate
    let mut table_sub_indicator = String::from("A");
    for f in file_vectors {
        // this minimal lenth is related to the checks we must do before performing a slice extraction
        if f.len() > 7 {
            // Since the header (IP0000T1) is the first thing to be read, and it contains the other table names
            // eg: IP0040T1, it will set the table_sub_indicator first and then it will be searched in all the file
            if &f[11..27] == "IP0000T1IP0040T1" {
                let s = f.as_str();
                table_sub_indicator.push_str(&s[243..246]);
            }

            // having len > 1, means it was filled by the if that is up here
            if table_sub_indicator.len() == 4 && &f[7..11] == &table_sub_indicator {
                println!("{}", f);
            }
        }
    }
    Ok(())
}
