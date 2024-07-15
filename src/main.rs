mod settings;

use glob::glob;

use settings::Settings;

fn main() {

    let _app_name = "vtask";

    use etcetera::{choose_base_strategy, BaseStrategy};

    let strategy = choose_base_strategy().unwrap();

    let _config_dir = strategy.config_dir().join(_app_name);
    let _data_dir = strategy.data_dir().join(_app_name);
    let _cache_dir = strategy.cache_dir().join(_app_name);


    println!("Configuration directory: {:?}", _config_dir);
    println!("Data directory: {:?}", _data_dir);

    let settings = Settings::new().unwrap();
    // Print out our settings (as a HashMap)
    println!( "{:?}", settings);


    let _path = String::from("c:/Users/aldrichtr/dendron/vaults/journal/notes/");
    let _pattern = String::from("task.*.md");
    let _target = _path + & _pattern;
    for entry in glob(& _target).expect("Failed to read glob pattern in settings") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
