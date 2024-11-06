use log::LevelFilter;

pub fn init_logger(log_level: LevelFilter) {
    env_logger::Builder::new().filter_level(log_level).init();
}
