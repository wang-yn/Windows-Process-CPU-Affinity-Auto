use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct ConfigWatcher {
    _watcher: RecommendedWatcher,
    receiver: Receiver<notify::Result<Event>>,
}

impl ConfigWatcher {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(Self {
            _watcher: watcher,
            receiver: rx,
        })
    }

    pub fn check_for_changes(&self) -> bool {
        // Check if there are any file change events
        while let Ok(Ok(event)) = self.receiver.try_recv() {
            use notify::EventKind;
            match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) => {
                    log::info!("Configuration file change detected");
                    return true;
                }
                _ => {}
            }
        }
        false
    }
}
