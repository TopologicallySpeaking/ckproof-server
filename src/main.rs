use std::fs::File;

use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::{error, get, web, App, Error, HttpRequest, HttpResponse, HttpServer};

use env_logger::Env;

use serde::Serialize;
use tera::Tera;

use ckproof::rendered::DocumentRendered;

const BIND_ADDR: Option<&str> = option_env!("BIND_ADDR");

#[derive(Serialize)]
struct PageData<Inner: Serialize> {
    title: String,
    inner: Inner,
    header_has_border: bool,
    footer_has_border: bool,
}

impl<Inner: Serialize> PageData<Inner> {
    fn to_response(self, template: &str, tera: web::Data<Tera>) -> Result<HttpResponse, Error> {
        let page_data_value = serde_json::to_value(self)
            .map_err(|_| error::ErrorInternalServerError("Data Serialization Error"))?;

        let ctx = tera::Context::from_value(page_data_value)
            .map_err(|_| error::ErrorInternalServerError("Template Context Error"))?;
        let s = tera
            .render(template, &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template Rendering Error"))?;

        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(s))
    }
}

#[get("/toc")]
async fn toc(
    tera: web::Data<Tera>,
    treatise: web::Data<DocumentRendered>,
) -> Result<HttpResponse, Error> {
    PageData {
        title: "Table of Contents".to_owned(),
        inner: treatise.manifest(),
        header_has_border: true,
        footer_has_border: true,
    }
    .to_response("toc.html.tera", tera)
}

#[get("/contributing")]
async fn contributing(tera: web::Data<Tera>) -> Result<HttpResponse, Error> {
    PageData {
        title: "Contributing".to_owned(),
        inner: (),
        header_has_border: true,
        footer_has_border: true,
    }
    .to_response("contributing.html.tera", tera)
}

#[get("/{book}/{chapter}/{page}")]
async fn page(
    tera: web::Data<Tera>,
    treatise: web::Data<DocumentRendered>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, Error> {
    let (book_id, chapter_id, page_id) = path.into_inner();
    let page = treatise
        .get_page(&book_id, &chapter_id, &page_id)
        .ok_or(error::ErrorNotFound("Page not found."))?;
    let title = page.name().to_owned();

    PageData {
        title,
        inner: page,
        header_has_border: true,
        footer_has_border: true,
    }
    .to_response("page.html.tera", tera)
}

#[get("/{book}/{chapter}")]
async fn chapter(
    tera: web::Data<Tera>,
    treatise: web::Data<DocumentRendered>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (book_id, chapter_id) = path.into_inner();
    let chapter = treatise
        .get_chapter(&book_id, &chapter_id)
        .ok_or(error::ErrorNotFound("Chapter not found."))?;
    let title = chapter.name().to_owned();

    PageData {
        title,
        inner: chapter,
        header_has_border: true,
        footer_has_border: true,
    }
    .to_response("chapter.html.tera", tera)
}

fn favicon(req: HttpRequest, filename: String) -> Result<HttpResponse, Error> {
    let path = format!("{}/static/favicon/{}", env!("CARGO_MANIFEST_DIR"), filename);

    let file = NamedFile::open(path)
        .map_err(|_| error::ErrorInternalServerError("Favicon matched but could not be opened."))?;

    file.into_response(&req)
}

fn book(
    tera: web::Data<Tera>,
    treatise: web::Data<DocumentRendered>,
    book_id: String,
) -> Result<HttpResponse, Error> {
    let book = treatise
        .get_book(&book_id)
        .ok_or(error::ErrorNotFound("Book not found."))?;
    let title = book.name().to_owned();

    PageData {
        title,
        inner: book,
        header_has_border: true,
        footer_has_border: true,
    }
    .to_response("book.html.tera", tera)
}

#[get("/{p}")]
async fn other(
    tera: web::Data<Tera>,
    treatise: web::Data<DocumentRendered>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let favicon_files = ["favicon.ico"];

    let p = req.match_info().get("p").unwrap().to_owned();

    if favicon_files.iter().any(|target| &p == *target) {
        favicon(req, p)
    } else {
        book(tera, treatise, p)
    }
}

#[get("/")]
async fn homepage(tera: web::Data<Tera>) -> Result<HttpResponse, Error> {
    PageData {
        title: "Online Treatise of Mathematical Proof".to_owned(),
        inner: (),
        header_has_border: true,
        footer_has_border: true,
    }
    .to_response("homepage.html.tera", tera)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        let f = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/rendered.json")).unwrap();
        let treatise: DocumentRendered = serde_json::from_reader(f).unwrap();

        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/*")).unwrap();
        let static_files =
            actix_files::Files::new("/static", concat!(env!("CARGO_MANIFEST_DIR"), "/static/"));

        App::new()
            .wrap(Logger::default())
            .data(treatise)
            .data(tera)
            .service(static_files)
            .service(toc)
            .service(contributing)
            .service(page)
            .service(chapter)
            .service(other)
            .service(homepage)
    })
    .bind(BIND_ADDR.unwrap_or("127.0.0.1:8000"))?
    .run()
    .await
}
