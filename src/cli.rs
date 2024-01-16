use rustyline::DefaultEditor;

pub fn get_user_input(prompt: &str) -> String {
    fn readline(prompt: &str) -> rustyline::Result<String> {
        let mut rl = DefaultEditor::new()?;

        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => Ok(line),
            Err(e) => {
                eprintln!("Error during readline: {:?}", e);
                std::process::exit(1);
            }
        }
    }
    readline(prompt).unwrap().trim().to_string()
}
