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
use std::path;

pub fn run(sensor_file: &str, db_file: &str, loop_interval: &i32) -> Result<(), Box<dyn Error>> {

    let sensor_contents = fs::read_to_string(sensor_file)?;

    println!("With text:\n{}", sensor_contents);

    Ok(())
}