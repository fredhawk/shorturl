use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct AlliumArgs {
    /// The long version of the url
    pub long_url: String,
    /// An optional short version of the url
    pub short_url: String,
}
