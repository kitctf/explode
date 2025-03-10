use clap::Parser;
use config::Config;
use explode::Explode;

pub mod explode;
pub mod init;
pub mod config;

fn main() {
    println!("Hello, world from explode!");

    let explode_command = Explode::parse();
    dbg!(explode_command);

    let config = Config::builder()
        .;

        todo!()
}
