mod args;

use args::AlliumArgs;
use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use is_url::is_url;

fn main() -> std::io::Result<()> {
    let args: AlliumArgs = AlliumArgs::parse();

    // Pull out the long url
    let _long = &args.long_url;

    // Pull out the short url
    let _short = &args.short_url.unwrap_or_default();

    // Check so that the long is a url
    let valid_url = is_url(_long);
    println!("{}", valid_url);

    // Make the entry from the strings
    let entry: String = "\n".to_owned() + _short + " " + _long;
    println!("{}", entry);

    // Check if file exists
    let file_exists = Path::new("_redirects").exists();
    if file_exists == true {
        // If it exists open it and append whatever is in the _long reference
        let mut f = File::options().append(true).open("_redirects")?;
        f.write(entry.as_bytes());
    } else {
        // If it doesnt exist. Create the file and write the _long reference to it.
        let mut f = File::create("_redirects")?;
        f.write(entry.as_bytes());
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
