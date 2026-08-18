#[derive(Debug)]
pub struct Config<'shared_config> {
    pub shared: &'shared_config crate::formatter::shared::config::Config,
}
