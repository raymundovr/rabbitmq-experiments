
pub mod configuration {
    use config::Config;
    use std::collections::HashMap;
    use anyhow::Result;

    pub fn load() -> Result<HashMap<String, String>> {
        let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()?;

        match settings.try_deserialize() {
            Ok(configuration) => Ok(configuration),
            Err(_) => panic!("Cannot read configuration file")
        }
    }
}

pub mod amqp_utils {
    use amiquip::Connection;
    use anyhow::Result;

    pub fn get_connection(url: &str) -> Result<Connection> {
        let connection = match url.starts_with("amqps") {
            true => Connection::open(url),
            false => Connection::insecure_open(url)
        }?;

        Ok(connection)
    }
}