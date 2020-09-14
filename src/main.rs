use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader};
use std::path::{Path, PathBuf};

use chrono::TimeZone;
use chrono_tz::America::New_York;
use log::{error, info};
use memchr::memchr;
use mysqldump_mutator::{InsertContext, Parser, SQLContextType};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "chump2csv")]
struct Opt {
    #[structopt(long)]
    no_unix_timestamp: bool,

    #[structopt(long, value_name = "SIZE", default_value = "1024")]
    stats_lru: usize,

    #[structopt(long, value_name = "OUT")]
    images: Option<PathBuf>,

    #[structopt(long, value_name = "OUT")]
    threads: Option<PathBuf>,

    #[structopt(long, value_name = "OUT")]
    daily: Option<PathBuf>,

    #[structopt(long, value_name = "OUT")]
    users: Option<PathBuf>,

    #[structopt(short, long, parse(from_os_str), default_value = "-")]
    output: PathBuf,

    #[structopt(name = "FILE", parse(from_os_str), default_value = "-")]
    input: PathBuf,
}

const GARBAGE: &'static [u8] = b"-- MySQL dump 10.13  Distrib 8.0.21, for osx10.15 (x86_64)
 --
 -- ------------------------------------------------------
 -- Server version	8

 ";

// mysqlchump inserts a BOM marker in the dumps
// right before the INSERT. It needs to be removed
// or the parser and mysql will choke on it.
pub struct BOMRemoveRead<T: Read> {
    inner: T,
}

impl<T: Read> BOMRemoveRead<T> {
    fn new(r: T) -> Self {
        Self { inner: r }
    }
}

impl<T: Read> Read for BOMRemoveRead<T> {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let result = self.inner.read(&mut buf)?;
        match memchr(0xEF, buf) {
            Some(p) if p + 2 < buf.len() => {
                if buf[p + 1] == 0xBB && buf[p + 2] == 0xBF {
                    buf.copy_within(p + 3.., p);
                    let result = result - 3;
                    Ok(result)
                } else {
                    Ok(result)
                }
            }
            _ => Ok(result),
        }
    }
}

fn convert_time(ny_time: i64) -> i64 {
    let ny_time = chrono::Utc
        .timestamp(ny_time as i64, 0)
        .with_timezone(&New_York);
    ny_time.naive_local().timestamp()
}

fn main() {
    pretty_env_logger::init_timed();
    let opt = Opt::from_args();

    let reader: BufReader<Box<dyn Read>> = if opt.input == Path::new("-") {
        info!("Reading from stdin");
        BufReader::new(Box::new(GARBAGE.chain(BOMRemoveRead::new(stdin()))))
    } else {
        info!("Reading from file {:?}", opt.input);
        BufReader::new(Box::new(GARBAGE.chain(BOMRemoveRead::new(
            File::open(&opt.input).expect("Failed to open file for reading"),
        ))))
    };

    let writer: Box<dyn Write> = if opt.output == Path::new("-") {
        Box::new(stdout())
    } else {
        Box::new(File::create(&opt.output).expect("Failed to open file for writing"))
    };

    let mut writer = csv::Writer::from_writer(writer);
    let mut prev_col = 0;
    let mut row_timestamp = 0;

    let err = Parser::parse_mysqldump(
        reader,
        |context, token| match context {
            SQLContextType::Insert(InsertContext::Value((_, column_index))) => {
                if *column_index < prev_col {
                    if !opt.no_unix_timestamp {
                        writer
                            .write_field(convert_time(row_timestamp).to_string())
                            .expect("failed to write to file");
                    }
                    writer
                        .write_record(None::<&[u8]>)
                        .expect("failed to write to file");
                }
                let tk_str = token.to_string();
                if tk_str == "NULL" {
                    writer.write_field(r"\N").expect("failed to write to file");
                } else {
                    let val = if tk_str.chars().next().unwrap() == '\'' {
                        &tk_str[1..tk_str.len() - 1]
                    } else {
                        tk_str.as_ref()
                    };
                    writer.write_field(val).expect("failed to write to file");
                }
                if *column_index == 4 && !opt.no_unix_timestamp {
                    row_timestamp = tk_str
                        .parse::<i64>()
                        .expect("failed to parse timestamp in column 4?");
                }
                prev_col = *column_index;
                token
            }
            SQLContextType::ColumnDefinition(_) => {
                // Here you can take note of what index is each column in each table.
                // println!("------- hjere -------");
                token
            }
            _ => {
                // panic!("DIE");
                // println!("h");
                token
            } // Or just return the original token
        },
        |_tokens| {},
    );

    if !opt.no_unix_timestamp {
        writer
            .write_field(convert_time(row_timestamp).to_string())
            .expect("failed to write to file");
    }
    writer
        .write_record(None::<&[u8]>)
        .expect("failed to write to file");

    if let Err(err) = err {
        error!("An error occured parsing the file: {}", err)
    }
}
