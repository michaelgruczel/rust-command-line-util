use clap::Parser;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "mgutil", version = "0.1.0", author = "Michael Gruczel")]
struct Args {
    /// command to be executed
    #[arg(short, long)]
    command: String,

    /// alias to be used
    /// e.g. an alias for a command or a folder
    #[arg(short, long)]
    alias: Option<String>,

    /// command or path, depending on command
    #[arg(short, long)]
    value: Option<String>,
}

struct Bookmark {
    alias: String,
    bookmark_type: String,
    value: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut bookmarks: Vec<Bookmark> = vec![];
    let command = args.command;
    let current_path = env::current_dir()?;
    let user_home_dir = std::env::home_dir();
    let user_home_dir_reference = &(user_home_dir.unwrap().display().to_string());
    let mgutil_bookmarks_file_reference =
        &(String::from(user_home_dir_reference) + "/.mgutil/bookmarks.csv");

    ensure_util_home_folder_exists(&user_home_dir_reference, mgutil_bookmarks_file_reference);
    load_existing_bookmarks(mgutil_bookmarks_file_reference, &mut bookmarks);

    match command.as_str() {
        // sp stands for save path
        "sp" => {
            if args.alias.is_none() {
                println!("the command safe path (sp) needs an alias");
            } else if args.value.is_some() {
                let alias = &args.alias.unwrap();
                let path = &args.value.unwrap();
                let bookmark_type = &String::from("path");
                println!("add path bookmark {} with specified path {}", alias, path);
                add_bookmark(mgutil_bookmarks_file_reference, alias, bookmark_type, path);
            } else {
                let alias = &args.alias.unwrap();
                let path = &current_path.display().to_string();
                let bookmark_type = &String::from("path");
                println!("add path bookmark {} with current path", alias);
                add_bookmark(mgutil_bookmarks_file_reference, alias, bookmark_type, path);
            }
        }
        // d stands for delete bookmark
        "d" => {
            if args.alias.is_none() {
                println!("the command delete (d) needs an alias");
            } else {
                let alias = &args.alias.unwrap();
                remove_bookmark(mgutil_bookmarks_file_reference, alias);
            }
        }
        // p stands for jump to path using a bookmark
        "p" => {
            if args.alias.is_none() {
                println!("the command path (p) needs an alias");
            } else {
                let alias = &args.alias.unwrap();
                open_shell_with_path(&bookmarks, alias);
            }
        }
        // l stands for list all bookmarks
        "l" => {
            println!("known bookmarks:");
            for bookmark in bookmarks {
                println!(
                    "alias '{}' has type '{}' and value {}",
                    bookmark.alias, bookmark.bookmark_type, bookmark.value
                )
            }
            println!("type bookmarks can be used with:    'mgutil --command p --alias <ALIAS>'");
            println!("command bookmarks can be used with: 'mgutil --command c --alias <ALIAS>'");
        }
        _ => println!("command {} not known", command),
    }

    Ok(())
}

fn add_bookmark(
    mgutil_bookmarks_file_reference: &String,
    alias: &String,
    bookmark_type: &String,
    value: &String,
) {
    let mgutil_bookmarks_file = File::options()
        .read(true)
        .write(true)
        .append(true)
        .open(mgutil_bookmarks_file_reference)
        .unwrap();
    let mut writer = BufWriter::new(mgutil_bookmarks_file);
    let line = String::from(alias)
        + ";"
        + &String::from(bookmark_type)
        + ";"
        + &String::from(value)
        + &String::from("\n");
    writer.write(line.as_bytes()).unwrap();
}

fn open_shell_with_path(bookmarks: &Vec<Bookmark>, target: &String) {
    for a_bookmark in bookmarks {
        if a_bookmark.alias.eq_ignore_ascii_case(&target) {
            println!(
                "match {}, open new iTerm terminal at {}",
                a_bookmark.alias, &a_bookmark.value
            );
            let mut cmd = Command::new("open");
            cmd.arg("-a").arg("iTerm").arg(&a_bookmark.value);
            let _ = cmd.output().expect("failed to execute process");
        }
    }
}

fn remove_bookmark(mgutil_bookmarks_file_reference: &String, alias: &String) {
    let mgutil_bookmarks_file = File::options()
        .read(true)
        .write(true)
        .append(true)
        .open(mgutil_bookmarks_file_reference)
        .unwrap();
    let reader = BufReader::new(&mgutil_bookmarks_file);
    let mut array: Vec<String> = vec![];
    let line_pattern = format!("{};", alias);
    print!("remove from aliases {}", &line_pattern);

    for line in reader.lines() {
        if let Ok(line) = line {
            if !line.starts_with(&line_pattern) {
                array.push(line);
            }
        }
    }
    File::create(mgutil_bookmarks_file_reference.as_str()).unwrap();
    // write the updated array back to the file
    let mut writer = BufWriter::new(mgutil_bookmarks_file);
    for line in array {
        writeln!(writer, "{}", line).unwrap();
    }
}

fn load_existing_bookmarks(
    mgutil_bookmarks_file_reference: &String,
    bookmarks: &mut Vec<Bookmark>,
) {
    let mgutil_bookmarks_file = File::open(mgutil_bookmarks_file_reference).unwrap();
    let reader = BufReader::new(mgutil_bookmarks_file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split(';');
            let alias = parts.next().unwrap();
            let bookmark_type = parts.next().unwrap();
            let value = parts.collect::<Vec<&str>>().join("");
            let bookmark = Bookmark {
                alias: String::from(alias),
                bookmark_type: String::from(bookmark_type),
                value: String::from(value),
            };
            bookmarks.push(bookmark);
        }
    }
}

fn ensure_util_home_folder_exists(
    user_home_dir_reference: &String,
    mgutil_bookmarks_file_reference: &String,
) {
    let _ = fs::create_dir_all(String::from(user_home_dir_reference) + "/.mgutil");
    if !!!Path::new(mgutil_bookmarks_file_reference.as_str()).exists() {
        File::create(mgutil_bookmarks_file_reference.as_str()).unwrap();
    }
}
