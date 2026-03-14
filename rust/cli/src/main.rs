mod cli;

fn main() {
    <cli::CLI as clap::Parser>::parse().execute();
}
