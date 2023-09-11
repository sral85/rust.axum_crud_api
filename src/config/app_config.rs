use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, env)]
    pub database_url: String,
}

pub fn parse() -> Config {
    Config::parse()
}
