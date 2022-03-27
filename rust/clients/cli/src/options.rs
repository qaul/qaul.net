/// Options for running the binary
/// 
/// Currently supported options:
/// 
/// * `-s, --storage-path`: Path to the storage directory
/// * `-h, --help`: Show help

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    /// Options to run the client
    #[structopt(flatten)]
    pub options: Options,
}

#[derive(StructOpt, Debug)]
pub struct Options {
    /// Path to the DB
    #[structopt(short, long = "storage-path", help = "Absolute path to the DB")]
    pub storage_path: Option<String>,
}
