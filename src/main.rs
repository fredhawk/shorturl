mod args;
mod lib;

use args::AlliumArgs;
use lib::rand_str;
use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use is_url::is_url;

fn main() -> std::io::Result<()> {
    let args: AlliumArgs = AlliumArgs::parse();

    // Pull out the long url
    let _long = &args.long_url;

    // Pull out the short url
    // If _short is nothing then call rand_str to create a random short url.
    let _short = &args.short_url.unwrap_or(rand_str());

    // Check so that the long is a url
    let valid_url = is_url(_long);

    if valid_url == true {
        // Make the entry from the strings
        let entry: String = "\n".to_owned() + "/" + _short + " " + "/" + _long;

        // Check if file exists
        let file_exists = Path::new("_redirects").exists();
        if file_exists == true {
            // If it exists open it and append whatever is in the _long reference
            let mut f = File::options().append(true).open("_redirects")?;
            let reader = BufReader::new(&f);
            let url_exist = reader.lines().any(|x| &x.unwrap() == _long);
            // for line in reader.lines() {
            if url_exist == true {
                panic!("URL already exists.");
            };
            // };

            f.write(entry.as_bytes())?;
        } else {
            // If it doesnt exist. Create the file and write the _long reference to it.
            let mut f = File::create("_redirects")?;
            f.write(entry.as_bytes())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let url = "https://example.com";
        let false_url = "nourl";

        assert_eq!(is_url(url), true);
        assert_eq!(is_url(false_url), false);
    }
}
