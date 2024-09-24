use std::{env, fs};

use zoxide::{config, db::Database, util::current_time};

fn main() {
    let args = Vec::from_iter(env::args());
    if args.len() != 3 {
        eprintln!("Usage: zoxide-edit <old> <new>");
        std::process::exit(1);
    }

    let now = current_time().unwrap();
    let db_path = config::data_dir().unwrap().join("db.zo");
    fs::copy(db_path.as_path(), db_path.with_extension("zo.bak")).unwrap();

    let mut db = Database::open().unwrap();
    db.with_dirs_mut(|dirs| {
        for dir in dirs.iter_mut() {
            dir.path = dir.path.replace(&args[1], &args[2]).into();
        }
    });
    // required to mark as dirty
    db.add_unchecked("/dev/null", 0.0, now);
    db.dedup();
    db.remove("/dev/null");

    db.save().unwrap()
}
