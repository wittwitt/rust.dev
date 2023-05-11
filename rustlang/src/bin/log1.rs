use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Naming};
use log::*;

mod a {
    pub mod aa {
        pub mod aaa {
            use log::*;
            pub fn say() {
                trace!("1");
                debug!("2");
                info!("3");
                warn!("4");
                error!("5");
            }
        }

        use log::*;
        pub fn say() {
            trace!("1");
            debug!("2");
            info!("3");
            warn!("4");
            error!("5");
        }
    }

    use log::*;
    pub fn say() {
        trace!("1");
        debug!("2");
        info!("3");
        warn!("4");
        error!("5");
    }
}

#[allow(dead_code)]
fn module_log() {
    // export RUST_LOG=trace,logg3=debug,logg3::a=trace,logg3::a::aa=warn,logg3::a::aa::aaa=error

    trace!("1");
    debug!("2");
    info!("3");
    warn!("4");
    error!("5");

    a::say();
    a::aa::say();
    a::aa::aaa::say();
}

fn main() {}

#[allow(dead_code)]
fn env_log() {
    flexi_logger::Logger::try_with_env()
        .unwrap()
        .start()
        .unwrap();

    flexi_logger::Logger::try_with_str("debug")
        .unwrap()
        .start()
        .unwrap();
}

#[allow(dead_code)]
fn build_log() {
    let mut log_builder = flexi_logger::LogSpecBuilder::new();

    log_builder.default(flexi_logger::LevelFilter::Trace);

    let mut logger = flexi_logger::Logger::with(log_builder.build())
        .start()
        .unwrap();

    // or update_log
    logger.set_new_spec(log_builder.build());
}

#[allow(dead_code)]
fn all_log() {
    // Logger::try_with_str("info, my::critical::module=trace")?.start()?;

    flexi_logger::Logger::try_with_env()
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("log_files") // create files in folder ./log_files
                .basename("foo"), // .discriminant("Sample4711A"), // use infix in log file name
                                  // .suffix("trc"), // use suffix .trc instead of .log
        )
        .rotate(
            // If the program runs long enough,
            Criterion::Age(Age::Day), // - create a new file every day
            Naming::Timestamps,       // - let the rotated files have a timestamp in their name
            Cleanup::Never,           // - keep at most 7 log files
        )
        .format_for_files(flexi_logger::detailed_format) // 代码行号
        // .duplicate_to_stderr(Duplicate::Warn) // print warnings and errors also to the console
        // .append() //  With FileSpec::suppress_timestamp you get a simple fixed filename, like foo.log.
        .start()
        .unwrap();
}
