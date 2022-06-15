mod remote_op;
mod adb;
mod ssh;
mod exec;
mod download;
mod term;
mod build_image;
mod cli;

fn main() {
    cli::run();
}