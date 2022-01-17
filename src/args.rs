use crate::constants;

pub struct Args {
    filename: String,
}

impl Args {
    pub fn new(env_args: Vec<String>) -> Result<Self, &'static str> {
        if env_args.len() > 1 {
            return Ok(Self {
                filename: env_args[1].to_owned(),
            });
        };
        Err("No files specified")
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn is_valid_file(&self) -> bool {
        self.filename.ends_with(constants::MANDO_FILE_EXTENSION)
    }
}
