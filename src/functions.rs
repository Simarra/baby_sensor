use std::fs;
use std::str::FromStr;
use std::path::PathBuf;

// use dotenv::dotenv;
use std::env;

// try to read the sensor file.
fn read_sensor_file(sensor_file: &std::path::PathBuf) -> Result<String, String> {
    match fs::read_to_string(sensor_file) {
        Ok(f_content) => Ok(f_content),
        Err(m_error) => panic!("Files doest not exists!"),
    }
}

// Extract temperature from a string object.
fn extract_temperature(sensor_content: &String) -> Result<f32, std::num::ParseFloatError> {
    println!("sensor file content: \n {}", sensor_content);
    let mut string_temp = sensor_content
        .lines()
        .nth(1)
        .unwrap()
        .split(" ")
        .nth(9)
        .unwrap();
    string_temp = &string_temp[2..];
    println!("str_temp is {}", string_temp);
    let res = f32::from_str(string_temp).unwrap() / 1000f32;
    Ok(res)
}

pub fn get_temperature(sensor_file: &std::path::PathBuf) -> f32 {
    let sensor_file = PathBuf::from(env::var("TEMP_FILE_PATH").expect("Temp File must be set"));
    let content = read_sensor_file(&sensor_file).unwrap();
    extract_temperature(&content).unwrap()

    // # Supprimer la premiere ligne qui est inutile
    // seconde_ligne = contenu.split("\n")[1]
    // donnees_temperature = seconde_ligne.split(" ")[9]
    // # Supprimer le "t=", et ajouter une virgule
    // return float(donnees_temperature[2:]) / 1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extracting() {
        let contents = String::from_str(
            "\
6E 50 f4 20 f5 05 J7 9U 9H : crc=90 YES
6E 50 f4 20 f5 05 J7 9U 9H t=6787",
        )
        .unwrap();
        assert_eq!(6.787f32, extract_temperature(&contents).unwrap())
    }
}
