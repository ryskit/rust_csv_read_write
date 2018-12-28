extern crate csv;

use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::env;
use csv::Writer;
use csv::Reader;

fn run() -> Result<(), Box<Error>> {
    let file_path = get_file_args()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    write(&mut rdr);
    Ok(())
}

fn get_file_args() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path)
    }
}

fn write(rdr: &mut Reader<File>) -> Result<(), Box<Error>> {
    let mut wtr = Writer::from_path("test.csv")?;
    for result in rdr.records() {
        let record = result?;
        wtr.write_record(record.get(0));
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
