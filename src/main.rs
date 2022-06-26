use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
struct CliConfig {

    /// Path for template file
    #[clap(short, long)]
    template: PathBuf,

    /// Path for file containing replacement values
    #[clap(short, long)]
    values: PathBuf,

    /// Path for output file
    #[clap(short, long)]
    output: PathBuf,

    /// Optional override for "app" param
    #[clap(short, long)]
    app: Option<String>,

    /// Optional override for "namespace" param
    #[clap(short, long)]
    namespace: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    replace: HashSet<ReplaceValue>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
struct ReplaceValue {
    param: String,
    value: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Read in CLI commands
    let cli: CliConfig = CliConfig::parse();

    // dbg!(cli.clone());

    //read in template
    let mut template = String::new();
    let mut tf = std::fs::File::open(cli.template)?;
    tf.read_to_string(&mut template)
        .expect("Could not read template file");

    // dbg!(template.clone());

    // read in values
    let vf = std::fs::File::open(cli.values)?;
    let values: Config = serde_yaml::from_reader(vf)?;

    // dbg!(values.clone());

    // Replace custom values from CLI first
    if let Some(app) = cli.app {
        template = template.replace("@@app@@", &*app);
    }
    if let Some(ns) = cli.namespace {
        template = template.replace("@@namespace@@", &*ns)
    }

    let replaces = values.replace;
    for rv in replaces {
        let rs = format!("@@{}@@", &*rv.param);
        template = template.replace(&*rs, &*rv.value);
    }

    // dbg!(template.clone());

    fs::write(cli.output, template)?;

    Ok(())

}
