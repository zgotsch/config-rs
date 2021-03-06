extern crate config;

use std::collections::HashMap;

fn main() {
    let settings = config::Config::default()
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();

    // Print out our settings (as a HashMap)
    println!("{:?}",
             settings.deserialize::<HashMap<String, String>>().unwrap());
}
