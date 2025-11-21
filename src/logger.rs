use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

fn current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

pub enum LockType {
    Read,
    Write,
}

pub enum LogMessage {
    Acquire(LockType),
    Release(LockType),
    Custom(String),
}

pub struct ThreadLogger {
    sender: Option<Sender<String>>,
    handle: Option<thread::JoinHandle<()>>,
    acquisitions: AtomicUsize,
    releases: AtomicUsize,
}

impl ThreadLogger {
    pub fn new(path: &str) -> Self {
        let (tx, rx) = mpsc::channel::<String>();

        // Spawn the actual logging thread
        let path = path.to_string();
        let handle = thread::spawn(move || logging_thread(rx, path));

        ThreadLogger {
            sender: Some(tx),
            handle: Some(handle),
            acquisitions: AtomicUsize::new(0),
            releases: AtomicUsize::new(0),
        }
    }

    pub fn log_id(&self, thread_id: u32, msg: LogMessage) {
        let timestamp = current_timestamp();

        let msg_string = match msg {
            LogMessage::Acquire(lock_type) => {
                self.acquisitions.fetch_add(1, Ordering::SeqCst);
                match lock_type {
                    LockType::Read => {
                        format!("{}: THREAD {} READ LOCK ACQUIRED\n", timestamp, thread_id)
                    }
                    LockType::Write => {
                        format!("{}: THREAD {} WRITE LOCK ACQUIRED\n", timestamp, thread_id)
                    }
                }
            }
            LogMessage::Release(lock_type) => {
                self.releases.fetch_add(1, Ordering::SeqCst);
                match lock_type {
                    LockType::Read => {
                        format!("{}: THREAD {} READ LOCK RELEASED\n", timestamp, thread_id)
                    }
                    LockType::Write => {
                        format!("{}: THREAD {} WRITE LOCK RELEASED\n", timestamp, thread_id)
                    }
                }
            }
            LogMessage::Custom(msg) => {
                format!("{}: THREAD {} {}\n", timestamp, thread_id, msg)
            }
        };

        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(msg_string);
        }
    }

    pub fn log_str<S: Into<String>>(&self, msg: S) {
        if let Some(sender) = self.sender.as_ref() {
            let _ = sender.send(msg.into());
        }
    }

    pub fn get_acquisition_count(&self) -> usize {
        self.acquisitions.load(Ordering::SeqCst)
    }
    pub fn get_release_count(&self) -> usize {
        self.releases.load(Ordering::SeqCst)
    }
}

fn logging_thread(rx: Receiver<String>, path: String) {
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);

    for msg in rx {
        writer.write_all(msg.as_bytes()).unwrap();
    }

    // When all senders are dropped, the loop ends and we flush/close the file
    writer.flush().unwrap();
}

impl Drop for ThreadLogger {
    fn drop(&mut self) {
        // Dropping the sender will close the channel and end the logging thread
        drop(self.sender.take());

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
