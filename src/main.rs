extern crate clap;
extern crate crypto;

use clap::{App, Arg, SubCommand};
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::{BufReader,Read,Write};
use std::io;
use std::process::{Command,Stdio};
use std::string::String;
use std::fs::{File,OpenOptions};
use std::fs::DirBuilder;
use std::fs::{read_dir, ReadDir};
use std::fs::rename;
use std::vec::Vec;
use std::iter::FromIterator;

fn read_md5(path: &str) -> io::Result<String> {
    // computes the md5 for the file
    let mut md5 = Md5::new();

    // feed filepath content
    let mut rf = BufReader::new(File::open(path)?);
    let mut dat = [0; 4024]; // XXX: hardcoded to 4K
    loop {
        match rf.read(&mut dat)? {
            0 => break,
            n => md5.input(&dat[0..n]),
        }
    }

    // returns the md5
    Ok(md5.result_str())
}

// TODO: use a std::path::Path instead
#[derive(Debug)]
struct RootTree {
    data_path : String,
    description_path : String,
}

// TODO: use a std::path::Path instead
#[derive(Debug)]
struct DataSet {
    hexmd5 : String,
    data_path : String,
    description_path : String,
}

fn dataset(root : &RootTree, md5 : &str) -> DataSet {
    let mut data = root.data_path.clone();
    data.push('/');
    data.push_str(md5);

    let mut description = root.description_path.clone();
    description.push('/');
    description.push_str(md5);

    DataSet {
        hexmd5: String::from(md5),
        data_path: data,
        description_path: description,
    }
}

fn make_roottree(root : &str) -> io::Result<RootTree> {
    let mut data = String::from(root);
    data.push_str("/data");
    DirBuilder::new()
        .recursive(true)
        .create(&data)?;

    let mut description = String::from(root);
    description.push_str("/description");
    DirBuilder::new()
        .recursive(true)
        .create(&description)?;

    Ok(RootTree { data_path: data, description_path: description })
}

fn format_tags(tags: &Vec<&str>) -> String {
    let mut tags_str = String::new();
    for (step,tag) in tags.into_iter().enumerate() {
        tags_str.push_str(tag);
        if 1 + step < tags.len() {
            tags_str.push(',');
        }
    };
    tags_str
}

fn edit_description(path : &str, dataset : &DataSet, tags: &Vec<&str>) {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&dataset.description_path);

    match file {
        Ok(mut f) => {
            f.write_all(
                // TODO use a real human-readable and parseable structure
                format!(include_str!("new-dataset.in"),
                    name = &path,
                    orig_path = &path,
                    tags = &format_tags(tags),
                    hexmd5 = &dataset.hexmd5).as_bytes())
        },
        // File exists, just re-open it.
        Err(_) =>
            Ok(()),
    }.ok();

    Command::new("vim") // TODO: pick EDITOR
        .arg(&dataset.description_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to finish vim");
}

fn move_datafile(path : &str, dataset : &DataSet) {
    let tgt = &dataset.data_path;
    rename(path, &tgt)
        .expect(&format!("could not move file to {}", tgt.as_str()));
}

fn scan_entries(entries : io::Result<ReadDir>, search : &str) -> io::Result<()> {
    // TODO: decouple with a iterating and printing with a filter
    for entry in entries? {
        let path = entry?.path();
        let mut buf_reader = BufReader::new(File::open(&path)?);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        if contents.contains(search) {
            //TODO: read data file metadata and print size as well
            println!("match: {}", path.to_str().unwrap_or("/dev/null"));
        }

    };
    Ok(())
}


// individual commands

fn run_add(root : &str, path : &str, tags : Vec<&str>) {
    let md5 = read_md5(path)
        .expect(&format!("could not compute md5 at {}", path));
    let root = make_roottree(root)
        .expect(&format!("could not make a root at {}", root));
    let dataset = dataset(&root, &md5);
    edit_description(path, &dataset, &tags);
    move_datafile(path, &dataset);
}

fn run_scan(root : &str, search : &str) {
    let root = make_roottree(root)
        .expect(&format!("could not make a root at {}", root));
    let read = read_dir(&root.description_path);
    scan_entries(read, search)
        .expect("could not scan description dir entirerly");
}


// main program

fn main() {
    let matches =
        App::new("dataset")
        .arg(Arg::with_name("root")
             .long("root")
             .help("root data dir")
             .takes_value(true))
        .subcommand(SubCommand::with_name("scan")
                    .about("scan datatsets")
                    .arg(Arg::with_name("search")
                         .short("s")
                         .long("search")
                         .help("fuzzy search")
                         .takes_value(true)
                         .required(true)))
        .subcommand(SubCommand::with_name("add")
                    .about("adds a new dataset")
                    .arg(Arg::with_name("dataset")
                         .short("ds")
                         .long("dataset")
                         .help("dataset file path")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("tag")
                         .short("t")
                         .long("tag")
                         .help("tag to add on a dataset")
                         .takes_value(true)
                         .multiple(true)))
        .get_matches();

    let root = matches.value_of("root").unwrap_or(".dataset");
    if let Some(add) = matches.subcommand_matches("add") {
        run_add(root,
               // "dataset" is required hence unwrap is legit
               add.value_of("dataset").unwrap(),
               // takes the tags
               FromIterator::from_iter(
                   add.values_of("tag").unwrap_or(Default::default())));
    } else if let Some(scan) = matches.subcommand_matches("scan") {
        run_scan(root,
                 // "search" is required, unwrap is legit
                 scan.value_of("search").unwrap());
    }
}
