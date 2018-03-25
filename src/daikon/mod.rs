use std::error::Error;
use std::fs::File;
use std::process::{self, Command};
use std::path::PathBuf;

pub mod invariants;
static DAIKON_INV_PATH: &str = "daikon.txt";
static DAIKON_DYNCOMP: &str = "daikon.DynComp";
static DAIKON_CHICORY: &str = "daikon.Chicory";

use super::Config;

pub fn setup_environment(
    config: &Config,
    output_path: &PathBuf,
    package: &str,
    class: &str,
) -> Result<(String, process::Command, process::Command), Box<Error>> {
    let invariants_out = String::from(output_path.join(DAIKON_INV_PATH).to_str().unwrap());

    // java -cp $CLASSPATH daikon.DynComp [package].[class]
    let mut dyncomp = Command::new("java");
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("-cp"));
    args.push(config.daikon_classpath.join(":"));
    args.push(String::from(DAIKON_DYNCOMP));
    args.push(format!("{}.{}", package, class));
    dyncomp.args(&args);

    // java -cp $CLASSPATH daikon.Chicory --daikon-online
    // --comparability-file=[class].decls-DynComp [package].[class]
    let mut chicory = Command::new("java");
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("-cp"));
    args.push(config.daikon_classpath.join(":"));
    args.push(String::from(DAIKON_CHICORY));
    args.push(String::from("--daikon-online"));
    args.push(String::from("--comparability-file"));
    args.push(format!("{}.decls-DynComp", class));
    args.push(format!("{}.{}", package, class));
    chicory.args(&args);

    let out_file = File::create(&invariants_out).unwrap();
    chicory.stdout(out_file);

    Ok((invariants_out, dyncomp, chicory))
}
