use std::path::Path;

use crossbeam_channel::Sender;
use notify::{Error, Event, EventKind, RecursiveMode, Watcher};
use quizmeet_rs_idl::quiz::Quiz;

use crate::parse_ods::read_from_file;

pub fn watch_for_files(path: &Path, sender: Sender<anyhow::Result<Quiz>>) -> anyhow::Result<()> {
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, Error>| match res {
        Ok(event) => {
            log::trace!("event: {:?}", event);
            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) => {
                    if let Some(path) = event.paths.first() {
                        let quiz = read_from_file(path);
                        sender.send(quiz).expect("sender disconnected");
                    } else {
                        log::error!(
                            "No paths created or modified on Create|Modify event: '{event:?}'??"
                        );
                    }
                }
                _ => (),
            }
        }
        Err(e) => log::error!("watch error: {:?}", e),
    })?;

    // watcher.watch(Path::new("../../ods/"), RecursiveMode::Recursive)?;
    watcher.watch(path, RecursiveMode::Recursive)?;

    Ok(())
}
