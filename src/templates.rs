use std::env;

use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let path = env::current_dir()
            .expect("Could not resolve current directory")
            .join("templates/**/*.{html,txt}");
        let tera = match Tera::new(path.to_str().unwrap()) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
