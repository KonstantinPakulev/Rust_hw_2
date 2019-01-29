#[macro_use]
extern crate serde_derive;
extern crate structopt;
extern crate csv;
extern crate serde;

use std::error::Error;
use std::io;
use std::process;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    city: String,
    population: Option<u64>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "params", about = "CLI params")]
struct Opt {
    #[structopt(name = "population-not-none", long)]
    population_not_none: bool,
    #[structopt(name = "population-none", long)]
    population_none: bool,
    #[structopt(name = "population-gt", default_value = "1000000", long)]
    population_gt: u64,
    #[structopt(name = "population-lt", default_value = "100000", long)]
    population_lt: u64,
    #[structopt(name = "out", long)]
    out: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = read(opt) {
        println!("{}", err);
        process::exit(1);
    }
}

fn read(params: Opt) -> Result<(), Box<Error>> {
    let mut reader = csv::Reader::from_path("pop.csv")?;

    let population_lt = params.population_lt;
    let population_gt = params.population_gt;

    let pop_none = params.population_none;
    let pop_not_none = !pop_none ;

    if params.out.is_none() {
        return write_std_out(&mut reader, &population_lt, &population_gt, &pop_none, &pop_not_none);
    }
    else {
        return write_file(&mut reader, &params.out.unwrap(), &population_lt, &population_gt, &pop_none, &pop_not_none);
    }
}

fn write_std_out(reader: &mut csv::Reader<std::fs::File>, population_lt: &u64, population_gt: &u64, pop_none: &bool, pop_not_none: &bool) -> Result<(), Box<Error>> {
    let mut writer = csv::WriterBuilder::new()
                .delimiter(b'\t')
                .quote_style(csv::QuoteStyle::Never)
                .from_writer(io::stdout());

    for result in reader.deserialize() {
        let record: Record = result?;
        if  *pop_not_none &&
            record.population.is_some() &&
            record.population.unwrap() > *population_lt &&
            record.population.unwrap() < *population_gt {
                writer.serialize(record)?;
                writer.flush()?;
        }
        else if *pop_none &&
                record.population.is_none() {
                writer.serialize(record)?;
                writer.flush()?;
        }
    }
    Ok(())
}

fn write_file(reader: &mut csv::Reader<std::fs::File>, path: &String, population_lt: &u64, population_gt: &u64, pop_none: &bool, pop_not_none: &bool) -> Result<(), Box<Error>> {
    let mut file_writer = csv::WriterBuilder::new()
                .delimiter(b'\t')
                .quote_style(csv::QuoteStyle::Never)
                .from_path(path)?;

    for result in reader.deserialize() {
        let record: Record = result?;
        if  *pop_not_none &&
            record.population.is_some() &&
            record.population.unwrap() > *population_lt &&
            record.population.unwrap() < *population_gt {
                file_writer.serialize(record)?;
                file_writer.flush()?;
        }
        else if *pop_none &&
                record.population.is_none() {
                file_writer.serialize(record)?;
                file_writer.flush()?;
        }
    }
    Ok(())
}
