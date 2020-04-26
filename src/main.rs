#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde_derive;

mod web;

use std::net::SocketAddr;

use clap::{App, Arg, SubCommand};
use glyph_bbox::dataset;
use warp::Filter;

fn cli_entrypoint<'b, 'a>() -> App<'a, 'b> {
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

#[tokio::main]
async fn main() {
    pretty_env_logger::init_custom_env("GLYPH_BBOX_RENDER_LOG_LEVEL");

    let rt = cli_entrypoint().get_matches();

    match rt.subcommand_name() {
        Some(v) => {
            let args = rt.subcommand_matches(v).unwrap();

            match v {
                "bbox" => {
                    let ds = dataset::DataSet::from_file(dataset::ReadOptions {
                        filename: args.value_of("dataset").unwrap().to_owned(),
                        format: dataset::Format::JSON,
                    });

                    let bbox = ds.bounding_box(
                        args.value_of("str").unwrap(),
                        dataset::BoundingBoxRenderOptions {
                            face: args.value_of("face").unwrap().to_owned(),
                            size: args.value_of("size").unwrap().to_owned(),
                        },
                    );

                    match bbox {
                        Some(v) => info!("{:?}", v),
                        None => error!("failed"),
                    }
                }
                "stat" => {
                    let ds = dataset::DataSet::from_file(dataset::ReadOptions {
                        filename: args.value_of("path").unwrap().to_owned(),
                        format: dataset::Format::JSON,
                    });

                    println!("{:#?}", ds);
                }
                "server" => {
                    let index_html =
                        warp::path::end().and_then(|| web::serve_file("index.html", "text/html"));
                    let main_js = warp::path("main.js")
                        .and_then(|| web::serve_file("main.js", "application/javascript"));
                    let raphael_js = warp::path("raphael.js").and_then(|| {
                        web::serve_file("vendor/raphael/raphael.min.js", "application/javascript")
                    });

                    let write = warp::post()
                        .and(warp::path!("write"))
                        .and(warp::query::<dataset::WriteOptions>())
                        .and(warp::body::json())
                        .and_then(web::write_dataset);

                    let bind_addr: SocketAddr = args
                        .value_of("bind")
                        .unwrap()
                        .parse()
                        .expect("Failed to parse bind address");

                    warp::serve(index_html.or(main_js).or(raphael_js).or(write))
                        .run(bind_addr)
                        .await;
                }
                _ => error!("unrecognized subcommand"),
            }
        }
        None => error!("no subcommand specified"),
    }
}
