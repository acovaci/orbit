// For DBT Cloud
// async fn version() {
//     let crate_version = clap::crate_version!();

//     let process = spawn_venv_command("dbt", &["version"]);

//     let stdout = process
//         .wait_with_output()
//         .await
//         .expect("failed to wait on child")
//         .stdout;

//     let output = String::from_utf8(stdout).expect("failed to parse output");

//     let re =
//         regex::Regex::new(r"dbt (?P<variant>.*) - (?P<version>\d+\.\d+\.\d+) \((?P<commit>[a-zA-Z\d]{40}) (?P<timestamp>\d+-\d+-\d+T\d+:\d+:\d+Z)\)")
//             .unwrap();

//     let captures = re.captures(&output).unwrap();

//     let variant = captures.name("variant").unwrap().as_str();
//     let version = captures.name("version").unwrap().as_str();
//     let commit = captures.name("commit").unwrap().as_str();
//     let timestamp = captures.name("timestamp").unwrap().as_str();

//     println!("Orbit version: {}", crate_version);
//     println!(
//         "DBT version:   {} ({} {} {})",
//         version, variant, commit, timestamp
//     );
// }

pub fn run(args: Vec<String>) {}
