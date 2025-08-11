use std::process::Command;
use std::fs::read_dir;

fn main() {

    let dirs = read_dir("./shared/shaders").unwrap();
    for file_name in dirs {
        match file_name {
            Ok(name) => {

                let original_name = name
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string();

                if !original_name.contains('.') { continue; }

                let words = original_name.split(".").collect::<Vec<&str>>();

                if words.len() != 2 { continue; }

                let format = words[0];
                let name = words[1];

                let out_name = format!("{}-{}.spv", name, format);

                if format == "frag" || format == "vert" {
                    let res = Command::new("glslc")
                        .arg(format!("./shared/shaders/{}", original_name))
                        .arg("-o")
                        .arg(format!("./shared/shaders/spv/{}", out_name))
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();

                    if !res.success() {
                        panic!("ERRR");
                    }
                }
            },

            _ => {}
        }
    }
}