mod cli;
mod github;
mod install;
mod platform;

fn main() {
    <cli::CLI as clap::Parser>::parse().execute();
}
