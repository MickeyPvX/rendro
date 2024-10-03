use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, ReadDir};
use std::io::Write;
use std::path::PathBuf;

use base64::{engine::general_purpose, Engine};
use minijinja::{context, Environment};

/// Base64 encodes an input String
fn base64_encode(input_str: String) -> String {
    general_purpose::STANDARD.encode(input_str)
}

/// Gets file paths at the given directory
fn get_file_paths(dir: &PathBuf) -> Result<ReadDir, String> {
    match fs::read_dir(dir) {
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => Err(format!(
                "Template file directory {} does not exist!",
                dir.to_string_lossy()
            )
            .into()),
            _ => Err(format!("Unexpected error reading templates input dir {:?}", err).into()),
        },
        Ok(read_dir) => Ok(read_dir),
    }
}

/// Generates the full path for the rendered template output file
fn generate_outfile_path(file_name: &PathBuf, out_dir: &PathBuf) -> Result<PathBuf, String> {
    let file_out_name = match file_name.extension() {
        Some(ext) => match ext.to_str().unwrap() {
            "j2" => file_name.file_stem().unwrap(),
            _ => file_name.as_path().as_os_str(),
        },
        None => {
            return Err(format!(
                "File name {:?} does not have a recognizeable file extension!",
                file_name
            )
            .into())
        }
    };

    Ok(out_dir.join(file_out_name))
}

/// Renders Jinja templates in 'input_dir' using environment variables,
/// saves the rendered templates to 'output_dir',
/// and will only render the files with the matching file extension.
pub fn render<'a>(
    input_dir: PathBuf,
    output_dir: PathBuf,
    file_extension: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut env = Environment::new();

    env.add_filter("b64encode", base64_encode);

    let file_paths = get_file_paths(&input_dir)?;

    match fs::create_dir(&output_dir) {
        Err(dir_err) => match dir_err.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => return Err(format!("Error creating directory: {:?}", dir_err).into()),
        },
        _ => (),
    };

    let mut env_hmap: HashMap<String, String> = HashMap::new();

    for (var, val) in std::env::vars() {
        env_hmap.insert(var.to_lowercase(), val);
    }

    for file_path in file_paths {
        let path = file_path?.path();
        let template_name = path.file_name().unwrap().to_str().unwrap().to_owned();

        if !path.is_dir() && path.extension() == Some(OsStr::new(&file_extension)) {
            env.add_template_owned(
                template_name,
                std::fs::read_to_string(&path)?,
            )?;
            let file_out_path = generate_outfile_path(&path, &output_dir)?;
            let template = env.get_template(path.file_name().unwrap().to_str().unwrap())?;

            let mut file_out = match fs::File::create(&file_out_path) {
                Err(file_err) => match file_err.kind() {
                    std::io::ErrorKind::AlreadyExists => fs::File::open(file_out_path)?,
                    _ => return Err(format!("Error creating output file: {:?}", file_err).into()),
                },
                Ok(file) => file,
            };

            println!("Rendering {:?}...", path.file_name().unwrap());

            file_out.write_all(template.render(context! {env => env_hmap})?.as_bytes())?;

            println!("Success!");
        }
    }

    Ok(())
}
