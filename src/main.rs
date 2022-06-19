mod remote_op;
mod adb;
mod ssh;
mod exec;
mod download;
mod term;
mod build_image;
mod cli;
mod constants;

fn main() {
    cli::run();
}