use std::collections::HashMap;
use std::path::Path;
use strum::IntoEnumIterator;
use walkdir::WalkDir;

use nuccbin::nucc_binary::{
    NuccBinaryParsed, NuccBinaryParsedDeserializer, NuccBinaryParsedReader, NuccBinaryParsedWriter,
};
use nuccbin::NuccBinaryType;

use xfbin::nucc::*;
use xfbin::nucc_chunk::*;
use xfbin::{read_xfbin, write_xfbin};

const NUCC_BINARY_PATTERNS: [NuccBinaryType; 3] = [
    NuccBinaryType::AccessoriesParam,
    NuccBinaryType::AccessoryParam,
    NuccBinaryType::MessageInfo,
];

//// Gather parsed NUCC binaries from a directory
pub fn get_nucc_binaries(directory: &Path) -> HashMap<NuccBinaryType, Box<dyn NuccBinaryParsed>> {
    let mut nucc_type_parsed = HashMap::new();

    let files = collect_files(&directory);

    files.iter().for_each(|file| {
        let xfbin = read_xfbin(&Path::new(file)).unwrap();
        for chunk in &xfbin.get_chunks_by_type(NuccChunkType::NuccChunkBinary) {
            let nucc_binary = chunk.downcast_ref::<NuccBinary>().unwrap();
            let data = nucc_binary.data.clone();
            if let Some(nucc_binary_type) = find_nucc_binary_type(&nucc_binary.struct_info.filepath)
            {
                if NUCC_BINARY_PATTERNS.contains(&nucc_binary_type) {
                    let reader = NuccBinaryParsedReader(nucc_binary_type, &data);
                    let nucc_binary_parsed: Box<dyn NuccBinaryParsed> = reader.into();
                    nucc_type_parsed.insert(nucc_binary_type, nucc_binary_parsed);
                }
            }
        }
    });

    if nucc_type_parsed.is_empty() {
        panic!("No valid NUCC binaries found in the directory!");
    }

    nucc_type_parsed
}

pub fn save_nucc_binaries(
    directory: &Path,
    nucc_binaries: &mut HashMap<NuccBinaryType, Box<dyn NuccBinaryParsed>>,
) {
    let files = collect_files(&directory);

    for file in &files {
        let mut xfbin = read_xfbin(&Path::new(file)).unwrap();

        let mut new_nucc_binaries: Vec<NuccBinary> = Vec::new();

        for chunk in xfbin.get_chunks_by_type(NuccChunkType::NuccChunkBinary) {
            let nucc_binary = chunk.downcast_ref::<NuccBinary>().unwrap();
            let chunk_info = nucc_binary.struct_info.clone();

            if let Some(nucc_binary_type) = find_nucc_binary_type(&chunk_info.filepath) {
                if let Some(nucc_binary_parsed) = nucc_binaries.get_mut(&nucc_binary_type) {
                    if NUCC_BINARY_PATTERNS.contains(&nucc_binary_type) {
                        let deserializer = NuccBinaryParsedDeserializer(
                            nucc_binary_type,
                            nucc_binary_parsed.serialize(),
                        );
                        let data = NuccBinaryParsedWriter(deserializer.into()).into();

                        let new_nucc_binary = NuccBinary {
                            struct_info: chunk_info,
                            version: 121,
                            data,
                        };

                        new_nucc_binaries.push(new_nucc_binary);
                    }
                }
            }
        }

        for n in new_nucc_binaries {
            for page in &mut xfbin.pages {
                for chunk in &mut page.structs {
                    if n.struct_info == chunk.downcast_mut::<NuccBinary>().unwrap().struct_info {
                        chunk.downcast_mut::<NuccBinary>().unwrap().data = n.data.clone();
                    }
                }
            }
        }

        write_xfbin(xfbin, &Path::new(file)).unwrap();
    }
}

fn find_nucc_binary_type(chunk_filepath: &String) -> Option<NuccBinaryType> {
    for nucc_binary_type in NuccBinaryType::iter() {
        let regex = nucc_binary_type.patterns();
        if regex.is_match(chunk_filepath) {
            return Some(nucc_binary_type);
        }
    }
    None
}

fn collect_files(directory: &Path) -> Vec<String> {
    let mut files = Vec::new();

    for entry in WalkDir::new(directory).follow_links(true) {
        match entry {
            Ok(entry) => {
                // Also only collect .xfbin files
                if entry.file_type().is_file() && entry.path().extension().unwrap() == "xfbin" {
                    files.push(entry.path().to_path_buf());
                }
            }
            Err(e) => eprintln!("Error accessing entry: {}", e),
        }
    }

    files
        .iter()
        .map(|path| path.to_str().unwrap().to_string())
        .collect::<Vec<String>>()
}
