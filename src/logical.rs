// LOOP:

// Get file / Db Path / loop_interval from CLI.
// Control each ones

// Read sensor file each every loop_inerval
// Extract temperature.
// Write it into Db
// If temp > 25' -> Send mail.

// API:

// Get current temp.
// Get n last days

use std::error::Error;

use std::fs;
use std::fs::File;
use std::io;
use std::path;
use std::{thread, time};

// fn extract_heatness(sensor_file: &str) -> Result<f32, io::Error> {
//     let mut s = String::new();
//     File::open(sensor_file)?.read_to_string(&mut s)?;
//     Ok(s);
// }

pub fn run_loop(
    sensor_file: &std::path::PathBuf,
    db_file: &std::path::PathBuf,
    loop_interval: &i32,
) -> Result<(), Box<dyn Error>> {
    // Run main loop

    loop {
        println!("{}", loop_interval);
        thread::sleep(time::Duration::from_secs(5));
    }
}
