use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let env_file = Path::new(".env");
    if !env_file.exists() {
        // Warn but don't crash if haven't made a local .env yet
        println!("cargo:warning=No .env file found by build.rs helper.");
        return;
    }

    if let Ok(file) = File::open(env_file) {
        let reader = BufReader::new(file);
        let mut env_vars = HashMap::new();

        for line in reader.lines().flatten() {
            let line = line.trim();
            if line.starts_with('#') || !line.contains('=') {
                continue;
            }
            if let Some((key, val)) = line.split_once('=') {
                let key = key.trim().to_string();
                let val = val.trim().trim_matches('"').trim_matches('\'').to_string();
                env_vars.insert(key, val);
            }
        }

        // Dynamically extract exact custom keys
        let user = env_vars.get("POSTGRES_USER").map(|s| s.as_str()).unwrap_or("postgres");
        let password = env_vars.get("POSTGRES_PASSWORD").map(|s| s.as_str()).unwrap_or("");
        let host = env_vars.get("POSTGRES_HOST").map(|s| s.as_str()).unwrap_or("localhost");
        let port = env_vars.get("POSTGRES_PORT").map(|s| s.as_str()).unwrap_or("5432");
        let db = env_vars.get("POSTGRES_DB").map(|s| s.as_str()).unwrap_or("postgres");

        // Generate the formatted connection string required by sqlx
        let database_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db);

        // This injects the variable into the compiler runtime macro memory space
        println!("cargo:rustc-env=DATABASE_URL={}", database_url);
    }

    println!("cargo:rerun-if-changed=.env");
}

