use std::path::Path;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::constants::{EADB_DIR, PROJECT_DIR};

use crate::{
    adb::Adb,
    download::download_file,
    remote_op::RemoteOp,
    ssh::Ssh,
    term::{print_err, print_tip}, build_image,
};


/// eBPF Android Debug Bridge - eadb
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Serial number of adb devices
    #[clap(long)]
    serial: Option<String>,

    /// Use ssh instead of adb to connect to the device
    #[clap(long)]
    ssh: Option<String>,

    /// the sshpass word to use
    #[clap(long)]
    sshpass: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Enter the eadb shell environment and get to work
    Shell,

    /// Remove eadb from the device
    Remove,

    /// Copy files from eadb filesystem to the device
    Push {
        /// source file in eadb filesystem
        src: String,
        /// target file in the device
        dst: String,
    },
    /// Copy files from the device to the eadb filesystem
    Pull {
        /// source file in the device
        src: String,
        /// target file in eadb filesystem
        #[clap(default_value_t = String::from("."))]
        dst: String,
    },

    /// Prepare the eadb environment for the device (when running the first time)
    /// By default, this will download and install a image on the device
    Prepare {
        /// Download and install the full image which contains compilers, editors, traces etc.
        #[clap(short, long)]
        full: bool,

        #[clap(short, long)]
        /// prepare envirnment with a local image instead of downloading it
        archive: Option<String>,

        /// Url to download prebuilt images
        #[clap(short = 'u', long, default_value_t = String::from("https://github.com/tiann/eadb/"))]
        image_url: String,

    },

    ///  Build and install the image
    Build {
        /// Use a specific temporary directory for build operation
        #[clap(long)]
        tempdir: Option<String>,

        /// Specify an ARCH to build for
        #[clap(long, default_value_t = String::from("arm64"))]
        arch: String,

        /// Debian distro to base on
        #[clap(long, default_value_t = String::from("bullseye"))]
        distro: String,

        /// Build and install BCC onto the device
        #[clap(long)]
        bcc: bool,

        /// mirror to use for debootstrap
        #[clap(long, default_value_t = String::from("http://ftp.us.debian.org/debian/"))]
        mirror: String,
    },
}

fn prepare_with_file(op: &dyn RemoteOp, workdir: &Path, file: &Path) -> Result<()> {
    print_tip(format!(
        "Using archive at {} for filesystem preparation",
        file.display()
    ));

    op.check_call(format!("mkdir -p {EADB_DIR}").as_str())?;

    print_tip("Pushing filesystem to device..");

    op.push(file.to_string_lossy().as_ref(), &format!("{}/deb.tar.gz", EADB_DIR))?;

    print_tip("Pushing addons to device..");

    extract_assets(op, workdir)?;

    print_tip("Unpacking filesystem in device..");

    op.check_call(&format!("{EADB_DIR}/device-unpack"))?;

    print_tip("All done! Run \"eadb shell\" to get started.");

    Ok(())
}

fn prepare_eadb(
    remote_op: &dyn RemoteOp,
    full: bool,
    image_url: String,
    archive: Option<String>,
) -> Result<()> {
    let working_dir = tempfile::tempdir()?;

    let working_path = working_dir.path();

    if let Some(archive_file) = archive {
        return prepare_with_file(remote_op, working_path, Path::new(&archive_file));
    }

    let download_url  = if full {
        format!("{}/releases/download/{}/debianfs-full.tar.gz", image_url, env!("CARGO_PKG_VERSION"))
    } else {
        format!("{}/releases/download/{}/debianfs-mini.tar.gz", image_url, env!("CARGO_PKG_VERSION"))
    };
    print_tip(format!("download image from: {}", download_url));

    let image_file = working_path.join("image.tar.gz");
    download_file(&download_url, image_file.as_path())?;
    prepare_with_file(remote_op, working_path, image_file.as_path())
}

fn remove_eadb(op: &dyn RemoteOp) -> Result<()> {
    op.check_call(&format!("{}/device-remove", EADB_DIR))?;
    Ok(())
}

fn extract_assets(op: &dyn RemoteOp, workdir: &Path) -> Result<()> {
    // create assets dir
    let assets_dir = workdir.join("assets");
    if assets_dir.exists() {
        std::fs::remove_dir_all(&assets_dir)?;
    }
    std::fs::create_dir_all(&assets_dir)?;

    PROJECT_DIR.extract(&assets_dir)?;

    let assets_path = assets_dir.to_string_lossy();
    let src_path = format!("{}/*", assets_path);

    op.push(&src_path, EADB_DIR)?;

    op.check_call(&format!("chmod +x {}/device-*", EADB_DIR))?;

    Ok(())
}

pub fn run() {
    let cli = Cli::parse();

    let remote_op: Box<dyn RemoteOp> = if let Some(ssh_uri) = &cli.ssh {
        Box::new(Ssh::new(ssh_uri, cli.sshpass))
    } else {
        Box::new(Adb::new(cli.serial))
    };

    if !matches!(cli.command, Commands::Build {..}) {
        if let Err(msg) = remote_op.check_connection() {
            print_err(format!("Cannot connect to the device: {}", msg));
            std::process::exit(1);
        }
    }

    let result = match cli.command {
        Commands::Shell => remote_op.shell(&format!("-t {}/run", EADB_DIR)),
        Commands::Pull { src, dst } => remote_op.pull(&src, &dst),
        Commands::Push { src, dst } => remote_op.push(&src, &dst),
        Commands::Remove => remove_eadb(&*remote_op),
        Commands::Build { tempdir, arch, distro, bcc, mirror } => {
            build_image::build(tempdir, arch, distro, bcc, mirror)
        }
        Commands::Prepare {
            full,
            image_url,
            archive,
        } => {
            prepare_eadb(&*remote_op, full, image_url, archive)
        }
    };

    if let Err(e) = result {
        print_err(format!("Error: {}", e));
    }
}
