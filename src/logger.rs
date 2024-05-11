use anyhow::{anyhow, Error};

static ANSI_DEBUG_COLOR: &str = "\x1b[0;37m";
static ANSI_ERROR_COLOR: &str = "\x1b[0;31m";
static ANSI_INFO_COLOR: &str = "\x1b[0;34m";
static ANSI_TRACE_COLOR: &str = "\x1b[0;35m";
static ANSI_WARN_COLOR: &str = "\x1b[0;33m";

static ANSI_RESET: &str = "\x1b[0m";

static FIVE_SPACES_INDENTATION: &str = "     ";
static FOUR_SPACES_INDENTATION: &str = "    ";

struct Logger;
static LOGGER: Logger = Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let ansi_color = match record.level() {
                log::Level::Debug => ANSI_DEBUG_COLOR,
                log::Level::Error => ANSI_ERROR_COLOR,
                log::Level::Info => ANSI_INFO_COLOR,
                log::Level::Trace => ANSI_TRACE_COLOR,
                log::Level::Warn => ANSI_WARN_COLOR,
            };
            let indentation = match record.level() {
                log::Level::Debug => FIVE_SPACES_INDENTATION,
                log::Level::Error => FIVE_SPACES_INDENTATION,
                log::Level::Info => FOUR_SPACES_INDENTATION,
                log::Level::Trace => FIVE_SPACES_INDENTATION,
                log::Level::Warn => FOUR_SPACES_INDENTATION,
            };
            println!(
                "{}{}╶╮\n{} │{}\n{}╶╯{}",
                ansi_color,
                record.level(),
                indentation,
                record
                    .args()
                    .to_string()
                    .replace('\n', &format!("\n{} │", indentation)),
                indentation,
                ANSI_RESET,
            );
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), Error> {
    log::set_logger(&LOGGER).map_err(|err| anyhow!(err))?;
    log::set_max_level(log::LevelFilter::Info);
    Ok(())
}
