
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