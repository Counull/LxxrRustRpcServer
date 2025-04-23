use std::ffi::OsStr;
use std::{env, fs, path::Path};
use std::path::PathBuf;

fn main() {
    let proto_src_dir = "proto";
    let proto_out_dir = "src/proto_gen";

    fs::create_dir_all(proto_out_dir).unwrap();
    let proto_files = collect_proto_files(&proto_src_dir);

    tonic_build::configure().out_dir(proto_out_dir).compile_protos(&proto_files, &[proto_src_dir]).unwrap();

}

fn collect_proto_files(proto_src_dir: &str) -> Vec<PathBuf> {
    let proto_dir = fs::read_dir(proto_src_dir).unwrap();

    proto_dir
        .filter_map(|entry| {
            let file_path = entry.unwrap().path();
            if file_path.extension().and_then(OsStr::to_str) == Some("proto") {
                Some(file_path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
