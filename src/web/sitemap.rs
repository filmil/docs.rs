

use iron::prelude::*;
use iron::headers::ContentType;
use super::page::Page;
use super::pool::Pool;
use time;


pub fn sitemap_handler(req: &mut Request) -> IronResult<Response> {
    let conn = extension!(req, Pool);
    let mut releases: Vec<(String, String)> = Vec::new();
    for row in &conn.query("SELECT DISTINCT ON (crates.name)
                                   crates.name,
                                   releases.release_time
                            FROM crates
                            INNER JOIN releases ON releases.crate_id = crates.id
                            WHERE rustdoc_status = true",
               &[])
        .unwrap() {
        releases.push((row.get(0), format!("{}", time::at(row.get(1)).rfc3339())));
    }
    let mut resp = ctry!(Page::new(releases).to_resp("sitemap"));
    resp.headers.set(ContentType("application/xml".parse().unwrap()));
    Ok(resp)
}


pub fn robots_txt_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with("Sitemap: https://docs.rs/sitemap.xml");
    resp.headers.set(ContentType("text/plain".parse().unwrap()));
    Ok(resp)
}
