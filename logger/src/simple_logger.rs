use slog::{o, Drain, FilterLevel, Logger, OwnedKVList, Record};
use slog_envlogger::LogBuilder;
use slog_scope::set_global_logger;
use std::{cell::RefCell, io, mem, sync::Mutex};
use termion::color::*;


pub fn init() {
    set_simple_logger("mirbft");
    set_simple_logger_prefix(format!("{}{}", Fg(LightYellow), Fg(Reset)))
}


struct SimpleLogger;

impl Drain for SimpleLogger {
    type Ok = ();
    type Err = io::Error;

    fn log(
        &self,
        record: &Record,
        _: &OwnedKVList,
    ) -> Result<Self::Ok, Self::Err> {
        let now = chrono::Local::now();
        PREFIX.with(|prefix| {
            let borrowed = prefix.borrow();
            let prefix = if let Some(ref prefix) = *borrowed {
                &prefix[..]
            } else {
                ""
            };
            println!(
                "{} {:<5} [{}:{}:{}] {} {}",
                now.format("%Y-%m-%d %H:%M:%S.%3f%z"),
                record.level(),
                record.module(),
                record.file(),
                record.line(),
                record.msg(),
                prefix,
            )
        });
        Ok(())
    }
}

thread_local! {
    pub static PREFIX: RefCell<Option<String>> = RefCell::new(None);
}


fn create_simple_logger(debug_module: &str) -> Logger {
    let drain = SimpleLogger.fuse();
    let mut builder = LogBuilder::new(drain);
    builder = builder.filter(None, FilterLevel::Info);
    builder = builder.filter(Some(debug_module), FilterLevel::Debug);

    if let Ok(s) = ::std::env::var("RUST_LOG") {
        builder = builder.parse(&s);
    }

    let envlogger = builder.build();

    Logger::root(Mutex::new(envlogger).fuse(), o!())
}

fn set_simple_logger(debug_module: &str) {
    mem::forget(set_global_logger(create_simple_logger(debug_module)));
}

fn set_simple_logger_prefix(id: String) {
    PREFIX.with(|x| x.replace(Option::Some(id)));
}
