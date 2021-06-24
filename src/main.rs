use clap::{app_from_crate, Arg};
use fstat::{run};
use fstat::options::Options;
use fstat::systems::FileSystem;
use std::sync::Arc;


fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::new("template")
                .about("Template for output")
                .short('t')
                .long("template")
                .required(false)
                .default_value("Size of {path} is {size_mb}mb"),
        )
        .arg(
            Arg::new("output")
                .about("Which entries to run output (may need to recurse regardless).\nroot = Just the entity specified by the path arg.\nall = All descendants of the entity specified by the path arg.")
                .short('o')
                .long("output")
                .required(false)
                .possible_values(&["root", "all"])
                .default_value("root"),
        )

        .arg("-v, --verbose 'Whether to print verbose logs'")
        .arg("-s, --single-thread 'Whether to avoid multi-threading'")
        .arg("<path> 'The file/folder path to check'")
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let output = matches.value_of_t("output").unwrap_or_else(|e| e.exit());

    let opts = Options {
        multithread: (matches.occurrences_of("single-thread") == 0),
        verbose: (matches.occurrences_of("verbose") > 0),
        output: output,
        template: String::from(matches.value_of("template").unwrap()),
    };

    let fs: Arc<dyn FileSystem>  = Arc::new(fstat::systems::fs::Fs {});
    run(path, opts, &fs);
}