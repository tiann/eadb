use std::path::Path;

use crate::constants::{
    DEFAULT_DEBIAN_DISTRO, DEFAULT_DEBIAN_REPO, DEFAULT_PREBUILT_ROOTFS_REPO, EADB_DIR, PROJECT_DIR, to_rootfs_dir,
};
use anyhow::Result;
use clap::{ArgGroup, Parser, Subcommand};

use crate::{
    adb::Adb,
    build_image,
    download::download_file,
    remote_op::RemoteOp,
    ssh::Ssh,
    term::{print_err, print_tip},
};

/// eBPF Android Debug Bridge - eadb
#[derive(Parser)]
#[clap(author, version, about)]
#[clap(group(ArgGroup::new("ssh_group").args(&["ssh"]).conflicts_with("adb_group")))]
#[clap(group(ArgGroup::new("adb_group").args(&["serial", "tcp-device", "usb-device"])))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Serial number of adb devices
    #[clap(short, long)]
    serial: Option<String>,

    /// ADB: use TCP/IP device (error if multiple TCP/IP devices available)
    #[clap(short = 'e')]
    tcp_device: bool,

    /// ADB: use USB device (error if multiple devices connected)
    #[clap(short = 'd')]
    usb_device: bool,

    /// Use ssh instead of adb to connect to the device
    #[clap(long)]
    ssh: Option<String>,

    /// the sshpass word to use
    #[clap(long, requires = "ssh", conflicts_with = "adb_group")]
    sshpass: Option<String>,

    /// the ssh port to use
    #[clap(short = 'p', long, requires = "ssh", conflicts_with = "adb_group")]
    port: Option<u32>,
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
        #[clap(short = 'u', long, default_value_t = String::from(DEFAULT_PREBUILT_ROOTFS_REPO))]
        image_url: String,

        #[clap(long, default_value_t = String::from("arm64"))]
        arch: String,
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
        #[clap(long, default_value_t = String::from(DEFAULT_DEBIAN_DISTRO))]
        distro: String,

        /// Build and install BCC onto the device
        #[clap(long)]
        bcc: bool,

        /// mirror to use for debootstrap
        #[clap(long, default_value_t = String::from(DEFAULT_DEBIAN_REPO))]
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

    op.push(
        file.to_string_lossy().as_ref(),
        &format!("{}/deb.tar.gz", EADB_DIR),
    )?;

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
    arch: String,
) -> Result<()> {
    let working_dir = tempfile::tempdir()?;

    let working_path = working_dir.path();

    if let Some(archive_file) = archive {
        return prepare_with_file(remote_op, working_path, Path::new(&archive_file));
    }

    let download_url = if full {
        format!(
            "{}/releases/download/v{}/debianfs-{}-full.tar.gz",
            image_url,
            env!("CARGO_PKG_VERSION"),
            arch
        )
    } else {
        format!(
            "{}/releases/download/v{}/debianfs-{}-mini.tar.gz",
            image_url,
            env!("CARGO_PKG_VERSION"),
            arch
        )
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
        Box::new(Ssh::new(ssh_uri, cli.sshpass, cli.port))
    } else {
        Box::new(Adb::new(cli.serial, cli.tcp_device, cli.usb_device))
    };

    if !matches!(cli.command, Commands::Build { .. }) {
        if let Err(msg) = remote_op.check_connection() {
            print_err(format!("Cannot connect to the device: {}", msg));
            std::process::exit(1);
        }
    }

    let result = match cli.command {
        Commands::Shell => remote_op.shell(&format!("-t {}/run", EADB_DIR)),
        Commands::Pull { src, dst } => remote_op.pull(&to_rootfs_dir(&src), &dst),
        Commands::Push { src, dst } => remote_op.push(&src, &to_rootfs_dir(&dst)),
        Commands::Remove => remove_eadb(&*remote_op),
        Commands::Build {
            tempdir,
            arch,
            distro,
            bcc,
            mirror,
        } => build_image::build(tempdir, arch, distro, bcc, mirror),
        Commands::Prepare {
            full,
            image_url,
            archive,
            arch,
        } => prepare_eadb(&*remote_op, full, image_url, archive, arch),
    };

    if let Err(e) = result {
        print_err(format!("Error: {}", e));
    }
}
