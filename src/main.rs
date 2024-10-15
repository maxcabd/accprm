mod cfg;
mod param;
mod png2xfbin;

use std::path::Path;

use cfg::Config;
use param::{add_entry::*, nucc_binary_handler::*};
use png2xfbin::png2xfbin;

use nuccbin::NuccBinaryType;
use xfbin::{read_xfbin, write_xfbin, NuccBinary};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "accprm",
    version = "0.1.0",
    author = "dei",
    about = "A tool to add accessory entries to NSC param files."
)]
struct Args {
    #[clap(short, long)]
    json: String,
    #[clap(short, long)]
    dir: String,
}

fn main() {
    let args = Args::parse();

    let cfg = Config::read_cfg(args.json.as_str());

    // Directory of data_win32
    let directory = Path::new(args.dir.as_str());

    let mut nucc_binaries = get_nucc_binaries(&directory);

    // Check if each required NUCC binary type is present in the directory
    let required_nucc_types = vec![
        NuccBinaryType::MessageInfo,
        NuccBinaryType::AccessoriesParam,
        NuccBinaryType::AccessoryParam,
    ];

    for nucc_type in &required_nucc_types {
        if !nucc_binaries.contains_key(nucc_type) {
            // Handle the case when the NUCC binary type is missing
            println!(
                "NUCC binary type {:?} is missing from the directory.",
                nucc_type
            );
        } else {
            match nucc_type {
                NuccBinaryType::MessageInfo => {
                    add_message_info_entry(&mut nucc_binaries, &cfg);
                }

                NuccBinaryType::AccessoriesParam => {
                    add_accessories_entry(&mut nucc_binaries, &cfg);
                }

                NuccBinaryType::AccessoryParam => {
                    add_accessory_entry(&mut nucc_binaries, &cfg);
                }

                _ => {}
            }
        }
    }

    save_nucc_binaries(&directory, &mut nucc_binaries);

    for accessory in &cfg.accessories {
        println!("Added accessory: {} [{}]", accessory.accessory_name, accessory.modelcode);

        let png_xfbin = png2xfbin(Path::new(&accessory.icon_filepath));
        let mut xfbin_path = directory.to_path_buf();
        xfbin_path.push("ui");
        xfbin_path.push("flash");
        xfbin_path.push("OTHER");
        xfbin_path.push("icon_acesories");

        // Create the directory ui/flash/OTHER/icon_acesories if it doesn't exist
        std::fs::create_dir_all(&xfbin_path).unwrap();

        xfbin_path.push(format!(
            "{}.xfbin",
            Path::new(&accessory.icon_filepath)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
        ));

        write_xfbin(png_xfbin, &xfbin_path).unwrap();
    }
}
