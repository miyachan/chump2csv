use std::borrow::Cow;
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

mod bom_remove;
mod stats;

use bom_remove::BOMRemoveRead;
use stats::WriteStat;

#[derive(StructOpt, Debug)]
#[structopt(name = "chump2csv")]
struct Opt {
    #[structopt(long, value_name = "delimiter", default_value = ",")]
    delimiter: String,

    #[structopt(long)]
    no_unix_timestamp: bool,

    #[structopt(long, value_name = "SIZE", default_value = "65536")]
    stats_lru: usize,

    #[structopt(long, value_name = "OUT")]
    images: Option<PathBuf>,

    #[structopt(long, value_name = "INDEX", default_value = "1")]
    images_start_index: u64,

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

#[derive(Default, Debug, Clone)]
pub struct Row {
    num: usize,
    thread_num: usize,
    op: bool,
    timestamp: i64,
    preview_orig: Option<String>,
    media_orig: Option<String>,
    media_hash: Option<String>,
    sticky: bool,
    locked: bool,
    name: Option<String>,
    trip: Option<String>,
    email: Option<String>,
}

impl Row {
    pub fn has_image(&self) -> bool {
        self.media_hash.is_some()
    }
}

struct StatBuilder<
    X: Write,
    T: stats::WriteStat<X> + for<'a> std::ops::AddAssign<&'a Row> + for<'a> From<&'a Row>,
> {
    lru: lru::LruCache<u64, T>,
    writer: csv::Writer<X>,
    images_start_index: u64,
}

impl<
        X: Write,
        T: stats::WriteStat<X> + for<'a> std::ops::AddAssign<&'a Row> + for<'a> From<&'a Row>,
    > StatBuilder<X, T>
{
    fn add_row(&mut self, row: &Row) -> csv::Result<()> {
        if let Some(key) = T::key(row) {
            if self.lru.contains(&key) {
                let x = self.lru.get_mut(&key).unwrap();
                x.add_assign(row);
            } else {
                if self.lru.len() == self.lru.cap() {
                    let out = self.lru.pop_lru().unwrap();
                    out.1.write(&mut self.writer, self.images_start_index)?;
                }
                let n = T::from(row);
                self.lru.put(key, n);
            }
        }
        Ok(())
    }
}

impl<
        X: Write,
        T: stats::WriteStat<X> + for<'a> std::ops::AddAssign<&'a Row> + for<'a> From<&'a Row>,
    > Drop for StatBuilder<X, T>
{
    fn drop(&mut self) {
        while let Some(x) = self.lru.pop_lru() {
            x.1.write(&mut self.writer, self.images_start_index)
                .expect("failed to write sql stats");
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
    let delimiter = match opt.delimiter.to_lowercase().as_str() {
        "tab" => b'\t',
        _ => {
            if opt.delimiter.len() != 1 {
                error!("Invalid delimiter, must be a single character");
                return;
            } else {
                opt.delimiter.chars().next().unwrap() as u8
            }
        }
    };

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

    let no_unix_timestamp = opt.no_unix_timestamp;
    let media_start_index = opt.images_start_index;
    let mut writer = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(writer);
    let mut prev_col = 0;
    let mut row = Row::default();
    let mut images = opt
        .images
        .as_ref()
        .map(|image_path| {
            let writer = csv::WriterBuilder::new()
                .delimiter(delimiter)
                .from_path(&image_path);
            writer.map(|writer| StatBuilder {
                lru: lru::LruCache::<u64, stats::Media>::new(opt.stats_lru),
                writer,
                images_start_index: media_start_index,
            })
        })
        .transpose()
        .expect("failed to open file for image sql");
    let mut threads = opt
        .threads
        .as_ref()
        .map(|thread_path| {
            let writer = csv::WriterBuilder::new()
                .delimiter(delimiter)
                .from_path(&thread_path);
            writer.map(|writer| StatBuilder {
                lru: lru::LruCache::<u64, stats::Thread>::new(opt.stats_lru),
                writer,
                images_start_index: media_start_index,
            })
        })
        .transpose()
        .expect("failed to open file for threads sql");
    let mut daily = opt
        .daily
        .as_ref()
        .map(|daily_path| {
            let writer = csv::WriterBuilder::new()
                .delimiter(delimiter)
                .from_path(&daily_path);
            writer.map(|writer| StatBuilder {
                lru: lru::LruCache::<u64, stats::Daily>::new(opt.stats_lru),
                writer,
                images_start_index: media_start_index,
            })
        })
        .transpose()
        .expect("failed to open file for daily sql");
    let mut users = opt
        .users
        .as_ref()
        .map(|users_path| {
            let writer = csv::WriterBuilder::new()
                .delimiter(delimiter)
                .from_path(&users_path);
            writer.map(|writer| StatBuilder {
                lru: lru::LruCache::<u64, stats::User>::new(opt.stats_lru),
                writer,
                images_start_index: media_start_index,
            })
        })
        .transpose()
        .expect("failed to open file for users sql");

    let err = Parser::parse_mysqldump(
        reader,
        |context, token| match context {
            SQLContextType::Insert(InsertContext::Value((_, column_index))) => {
                if *column_index < prev_col {
                    if let Some(images) = images.as_mut() {
                        images
                            .add_row(&row)
                            .expect("failed to write sql stats for images");
                    }
                    if let Some(threads) = threads.as_mut() {
                        threads
                            .add_row(&row)
                            .expect("failed to write sql stats for threads");
                    }
                    if let Some(daily) = daily.as_mut() {
                        daily
                            .add_row(&row)
                            .expect("failed to write sql stats for daily");
                    }
                    if let Some(users) = users.as_mut() {
                        users
                            .add_row(&row)
                            .expect("failed to write sql stats for users");
                    }

                    if media_start_index == 0 || images.is_none() || !row.has_image() {
                        writer.write_field(b"0").expect("failed to write to file");
                    } else {
                        let images = images.as_ref().unwrap();
                        let key = <stats::Media as WriteStat<&mut [u8]>>::key(&row).unwrap();
                        let media = images.lru.peek(&key).unwrap();
                        let media_id = media.media_id + media_start_index - 1;
                        writer
                            .write_field(media_id.to_string())
                            .expect("failed to write to file");
                    }

                    // row finished
                    if !no_unix_timestamp {
                        writer
                            .write_field(convert_time(row.timestamp).to_string())
                            .expect("failed to write to file");
                    }
                    writer
                        .write_record(None::<&[u8]>)
                        .expect("failed to write to file");

                    row = Row::default();
                }
                let tk_str = token.to_string();
                let val = if tk_str == "NULL" {
                    Cow::Borrowed(r"\N")
                } else {
                    let val = if tk_str.chars().next().unwrap() == '\'' {
                        if tk_str == "'NULL'" {
                            tk_str.as_ref()
                        } else {
                            &tk_str[1..tk_str.len() - 1]
                        }
                    } else {
                        tk_str.as_ref()
                    };

                    let val = match memchr(b'"', val.as_bytes()) {
                        Some(_) => Cow::Owned(val.replace("\\\"", "\"")),
                        None => Cow::Borrowed(val),
                    };

                    match memchr(b'\'', val.as_bytes()) {
                        Some(_) => Cow::Owned(val.replace("\\'", "'")),
                        None => val,
                    }
                };

                writer
                    .write_field(val.as_ref())
                    .expect("failed to write to file");

                match *column_index {
                    0 => {
                        row.num = tk_str.parse().expect("corrupt column 0");
                    }
                    2 => {
                        row.thread_num = tk_str.parse().expect("corrupt column 2");
                    }
                    3 => {
                        row.op = tk_str.parse::<usize>().expect("corrupt column 3") == 1;
                    }
                    4 => {
                        row.timestamp = tk_str.parse::<i64>().expect("corrupt column 4");
                    }
                    6 => {
                        row.preview_orig = if val == r"\N" {
                            None
                        } else {
                            Some(String::from(val))
                        }
                    }
                    13 => {
                        row.media_hash = if val == r"\N" {
                            None
                        } else {
                            Some(String::from(val))
                        }
                    }
                    14 => {
                        row.media_orig = if val == r"\N" {
                            None
                        } else {
                            Some(String::from(val))
                        }
                    }
                    18 => {
                        row.email = if val == r"\N" {
                            None
                        } else {
                            Some(String::from(val))
                        }
                    }
                    19 => {
                        row.name = if val == r"\N" {
                            None
                        } else {
                            Some(String::from(val))
                        }
                    }
                    20 => {
                        row.trip = if val == r"\N" {
                            None
                        } else {
                            Some(String::from(val))
                        }
                    }
                    23 => {
                        row.sticky = tk_str.parse::<usize>().expect("corrupt column 22") == 1;
                    }
                    24 => {
                        row.locked = tk_str.parse::<usize>().expect("corrupt column 23") == 1;
                    }
                    _ => (),
                };
                prev_col = *column_index;
                token
            }
            _ => token,
        },
        |_tokens| {},
    );

    if let Err(err) = err {
        error!("An error occured parsing the sql file: {}", err)
    }

    if let Some(images) = images.as_mut() {
        images
            .add_row(&row)
            .expect("failed to write sql stats for images");
    }
    if let Some(threads) = threads.as_mut() {
        threads
            .add_row(&row)
            .expect("failed to write sql stats for threads");
    }
    if let Some(daily) = daily.as_mut() {
        daily
            .add_row(&row)
            .expect("failed to write sql stats for daily");
    }
    if let Some(users) = users.as_mut() {
        users
            .add_row(&row)
            .expect("failed to write sql stats for users");
    }

    // row finished

    if media_start_index == 0 || images.is_none() || !row.has_image() {
        writer.write_field(b"0").expect("failed to write to file");
    } else {
        let images = images.as_ref().unwrap();
        let key = <stats::Media as WriteStat<&mut [u8]>>::key(&row).unwrap();
        let media = images.lru.peek(&key).unwrap();
        let media_id = media.media_id + media_start_index - 1;
        writer
            .write_field(media_id.to_string())
            .expect("failed to write to file");
    }

    if !no_unix_timestamp {
        writer
            .write_field(convert_time(row.timestamp).to_string())
            .expect("failed to write to file");
    }
    writer
        .write_record(None::<&[u8]>)
        .expect("failed to write to file");
}
