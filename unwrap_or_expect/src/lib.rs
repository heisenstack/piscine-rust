pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}
//String::from()
pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
       Security::Unknown => server.unwrap().to_string(),
       Security::Message => server.expect("ERROR: program stops").to_string(),
       Security::Warning => match server {
        Ok(data) => String::from(data),
        Err(_) => "WARNING: check the server".to_string(),
       }
       Security::NotFound => match server {
        Ok(data) => String::from(data),
        Err(err) => format!("Not found: {}", err),
       }
       Security::UnexpectedUrl => match server {
        Ok(data) => panic!("Unexpected URL: {}", data),
        Err(err) => err.to_string(),
       },
    //    _ => String::from("Yaaow!")
    }
}