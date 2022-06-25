use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    template: String,
    output: String,
    replace: Vec<ReplaceValue>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ReplaceValue {
    param: String,
    value: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO add clap or similar

    //read in template
    let mut template = String::new();
    let mut tf = std::fs::File::open("template.yaml")?;
    tf.read_to_string(&mut template)
        .expect("Could not read file");

    dbg!(template.clone());

    // read in values
    let mut vf = std::fs::File::open("values.yaml")?;
    let mut values: Config = serde_yaml::from_reader(vf)?;

    dbg!(values.clone());

    let replaces = values.replace;
    for rv in replaces {
        template = template.replace(&*rv.param, &*rv.value);
    }

    dbg!(template.clone());

    fs::write("output.yaml", template)?;

    Ok(())

}
