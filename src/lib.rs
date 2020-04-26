#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod web;

use clap::{App, Arg, SubCommand};

pub fn cli_entrypoint<'b, 'a>() -> App<'a, 'b> {
    App::new("glyph-bbox-render")
        .version(crate_version!())
        .author("Mihir Singh (@citruspi)")
        .subcommand(
            SubCommand::with_name("server")
                .about("run rendering server")
                .arg(
                    Arg::with_name("bind")
                        .takes_value(true)
                        .short("b")
                        .long("bind")
                        .help("address to bind to")
                        .default_value("127.0.0.1:2352"),
                ),
        )
        .subcommand(
            SubCommand::with_name("stat").about("inspect data set").arg(
                Arg::with_name("path")
                    .help("path of the dataset to inspect")
                    .takes_value(true)
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("bbox")
                .about("calculate bounding box")
                .arg(
                    Arg::with_name("dataset")
                        .help("dataset path")
                        .takes_value(true)
                        .short("d")
                        .long("data-set")
                        .required(true),
                )
                .arg(
                    Arg::with_name("face")
                        .help("font face")
                        .takes_value(true)
                        .short("f")
                        .long("face")
                        .required(true),
                )
                .arg(
                    Arg::with_name("size")
                        .help("font size")
                        .takes_value(true)
                        .short("s")
                        .long("size")
                        .required(true),
                )
                .arg(
                    Arg::with_name("str")
                        .help("string")
                        .takes_value(true)
                        .index(1)
                        .required(true),
                ),
        )
}
