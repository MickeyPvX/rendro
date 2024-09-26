pub mod cli;

pub const TEMPLATES_PATH: &str = "./src/templates/";
pub const OUTPUT_DIR: &str = "./src/templates/temp";

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::cli::render;

    use super::*;

    #[test]
    fn test_render_bad_input_dir() {
        let result: Result<(), Box<dyn std::error::Error>> = render(
            PathBuf::from("not_a_dir/"),
            PathBuf::from(OUTPUT_DIR),
            String::from(".j2"),
        );

        assert!(result.is_err())
    }
}
