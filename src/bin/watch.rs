extern crate notify;

use glob::glob;
use notify::{watcher, DebouncedEvent::*, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("ods", RecursiveMode::Recursive).unwrap();
    // for entry in glob("ods/*.ods").unwrap().filter_map(Result::ok) {
    //     // self.open(entry.as_path())?;
    //     watcher
    //         .watch(entry.as_path(), RecursiveMode::Recursive)
    //         .unwrap();
    // }

    loop {
        match rx.recv() {
            Ok(event) => {
                match &event {
                    NoticeWrite(p) | NoticeRemove(p) | Create(p) | Write(p) | Chmod(p)
                    | Remove(p) => {
                        dbg!(p);
                        if !check_ext(p) {
                            continue;
                        }
                        // if let Some(ext) = p.extension() {
                        //     if ext != "ods" {
                        //         continue;
                        //     }
                        // }
                    }
                    _ => {
                        println!("other");
                        // continue;
                    }
                };

                dbg!(event);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn check_ext(p: &PathBuf) -> bool {
    if let Some(ext) = p.extension() {
        if ext != "ods" {
            return false
        }
    }
    true
}
