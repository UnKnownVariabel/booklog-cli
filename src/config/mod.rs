use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub book_file: String,
}

pub fn init_config() {
    let loaded_config = Config {
        book_file: "books.csv".to_string(),
    };

    CONFIG
        .set(loaded_config)
        .expect("Error: trying to init config twice");
}

pub fn get_config() -> &'static Config {
    CONFIG
        .get()
        .expect("Error: trying to acces non initialized config!")
}
