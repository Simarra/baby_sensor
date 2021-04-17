use structopt::StructOpt;
#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;

mod db;
mod api;
use db::*;

mod baby_loop;

#[derive(StructOpt)]
struct Cli {
    /// The path to the sensor file to read
    /// should be /sys/bus/w1/devices/
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
    // baby_loop::run_loop(&args.sensor_file, &args.db_file, &args.loop_interval); // MAKE IT ASYNC
    api::main();
    // println!("Hi guy! {}", &args.loop_interval);
    return ();
}
