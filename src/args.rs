use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author="Fred", version, about="URL shortener")]
pub struct AlliumArgs {
    /// The long version of the url
    #[clap(short='l', long="long")]
    pub long_url: String,

    /// An optional short version of the url
    #[clap(short='s', long="short")]
    pub short_url: Option<String>,
}
