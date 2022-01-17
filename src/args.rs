pub struct Args {
    filename: String,
}

impl Args {
    pub fn new(env_args: Vec<String>) -> Result<Args, &'static str> {
        if env_args.len() > 1 {
            return Ok(Args {
                filename: env_args[1].to_owned(),
            });
        };
        Err("No files specified")
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }
}
