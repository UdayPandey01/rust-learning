#[derive(Debug)]
struct Config {
    username: String,
    port: u16,
}

#[derive(Debug)]
enum ParseError {
    MissingUsername,
    MissingPort,
    InvalidPort,
}

// TODO: Implement this function
// It should parse a string like "username=test\nport=8080"
fn parse_config(s: &str) -> Result<Config, ParseError> {
    let mut username = None;
    let mut port = None;

    for line in s.lines() {
        if line.starts_with("username=") {
            username = Some(line.trim_start_matches("username=").to_string());
        } else if line.starts_with("port=") {
            let port_str = line.trim_start_matches("port=");
            // The `parse()` method on a string returns a Result.
            // If it's Ok, assign it. If it's Err, we have an InvalidPort error.
            match port_str.parse::<u16>() {
                Ok(p) => port = Some(p),
                Err(_) => return Err(ParseError::InvalidPort),
            }
        }
    }
    
    // After the loop, check if we found both a username and a port.
    // If username is None, return Err(ParseError::MissingUsername)
    // If port is None, return Err(ParseError::MissingPort)
    
    match (username, port) {
        (Some(u), Some(p)) => Ok(Config {
            username : u,
            port : p
        }),
        (None, _) => return Err(ParseError::MissingUsername),
        (_, None) => return Err(ParseError::MissingPort)
    }
    
    // If both are Some, create a Config struct and return Ok(config).
    // You can use a `match` on the tuple `(username, port)`.

}

fn main() {
    let config_str = "username=uday\nport=8080";
    match parse_config(config_str) {
        Ok(config) => println!("Parsed config successfully: {:?}", config),
        Err(e) => println!("Error parsing config: {:?}", e),
    }

    let invalid_config_str = "username=test\nport=abc";
    match parse_config(invalid_config_str) {
        Ok(config) => println!("Parsed config successfully: {:?}", config),
        Err(e) => println!("Error parsing config: {:?}", e),
    }
}