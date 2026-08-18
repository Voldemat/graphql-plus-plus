#[derive(Debug)]
pub struct Config<'config> {
    pub shared: &'config crate::formatter::shared::config::Config,
}
