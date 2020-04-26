use rust_embed::RustEmbed;
use warp::{http::HeaderValue, reply::Response, Rejection, Reply};

use glyph_bbox::dataset;

#[derive(RustEmbed)]
#[folder = "assets/web"]
struct Asset;

pub async fn serve_file(path: &str, content_type: &str) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;

    let mut res = Response::new(asset.into());
    res.headers_mut()
        .insert("Content-Type", HeaderValue::from_str(content_type).unwrap());
    Ok(res)
}

pub async fn write_dataset(
    opts: dataset::WriteOptions,
    dataset: dataset::DataSet,
) -> Result<impl Reply, Rejection> {
    let res: Response;
    let r = dataset.write(opts);

    match r {
        Ok(()) => {
            res = Response::new("success".into());
        }
        Err(e) => {
            error!("{:?}", e);
            res = Response::new("failed to write dataset".into());
        }
    }

    Ok(res)
}
