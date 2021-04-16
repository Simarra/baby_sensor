
use structopt::StructOpt;

mod logical;

#[derive(StructOpt)]
struct Cli {
    /// The path to the sensor file to read
    #[structopt(parse(from_os_str))]
    sensor_file: std::path::PathBuf,

    /// The path to the sqlite database.
    #[structopt(parse(from_os_str))]
    db_file: std::path::PathBuf,

    /// Loop interval
    #[structopt(default_value = "42")]
    loop_interval: i32,
}

fn main() {
    let args = Cli::from_args();
    // logical::run(args., db_file, loop_interval)
}
