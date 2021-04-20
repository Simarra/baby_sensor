use std::path;
use std::fs;
use std::io;


// try to read the sensor file.
fn read_sensor_file(sensor_file: &std::path::PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(sensor_file)
}

// Extract temperature from a string object.
fn extract_temperature(sensor_content: &String) -> Result<f32, io::Error> {
    let string_temp = sensor_content.lines().nth(2)?.split(" ").nth(9)?;
    f32::from(string_temp);
}

pub fn get_temperature(sensor_file: &std::path::PathBuf) -> Result<f32, Box<dyn Error>> {
    match extract_temperature(read_sensor_file(&sensor_file)){
        Ok(result) => result,
        Error() => 0.0
    }

    // # Supprimer la premiere ligne qui est inutile
    // seconde_ligne = contenu.split("\n")[1]
    // donnees_temperature = seconde_ligne.split(" ")[9]
    // # Supprimer le "t=", et ajouter une virgule
    // return float(donnees_temperature[2:]) / 1000
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extracting() {
        let contents = "\
6E 50 f4 20 f5 05 J7 9U 9H : crc=90 YES
6E 50 f4 20 f5 05 J7 9U 9H t=6787;"
        assert_eq!("6.87", extract_temperature(sensor_file: &String)(query, contents));
    }

}