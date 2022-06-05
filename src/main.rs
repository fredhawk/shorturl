mod args;

use args::AlliumArgs;
use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::io::Write;


fn main() -> std::io::Result<()> {
    let args: AlliumArgs = AlliumArgs::parse();

    // Pull out the long url
    let _long = &args.long_url;

    // Check if file exists
    let mut file_exists:bool = false;
    file_exists = Path::new("_redirects").exists();
    if file_exists == true {
        // If it exists open it and append whatever is in the _long reference
        let mut f = File::options().append(true).open("_redirects")?;
        f.write(_long.as_bytes());
    } else {
        // If it doesnt exist. Create the file and write the _long reference to it.
        let mut f = File::create("_redirects")?;
        f.write(_long.as_bytes());
    }

    //TODO 1. add both variables to the file
    //TODO 2. add a new line first before the strings
    //TODO 3. add spacing between the strings
    //TODO 4. Make a program that encodes strings
    //TODO 5. Check if the program gets 1 or 2 args. If only one then create an encoded string
    // and use that as second argument when writing to file.

    // println!("{}", long);
    Ok(())
}
