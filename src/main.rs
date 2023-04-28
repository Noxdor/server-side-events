use actix_files as fs;
use actix_web::Error;
use actix_web::{get, web::Bytes, App, HttpResponse, HttpServer, Responder};
use futures::channel::mpsc;
use futures::SinkExt;

// define ip and port of the server sending the events
const IP: &str = "localhost";
const PORT: u16 = 8080;

// route that responds with event stream
#[get("/progress")]
async fn sse() -> impl Responder {
    // create mpsc channel to send an update every 100ms
    let (mut tx, rx) = mpsc::channel::<Result<Bytes, Error>>(0);
    // create a thread that sends data in the required format
    actix_web::rt::spawn(async move {
        for i in 0..=100 {
            println!("loop {}", i);
            tx.send(Ok(Bytes::from(format!(
                "id: {}\nevent: progress\ndata: {{\"progress\":{}}}\n\n",
                i, i
            ))))
            .await
            .unwrap();

            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        tx.send(Ok(Bytes::from(format!(
            "id: 101\nevent: close\ndata: {{}}\n\n"
        ))))
        .await
        .unwrap();
        tx.close().await.unwrap();
    });

    HttpResponse::Ok()
        // this content type makes the response an event stream
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        // stream receiving the updates from the sending thread
        .streaming(rx)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // create server instance
    let server = HttpServer::new(|| {
        App::new()
            .service(sse)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind((IP, PORT))?
    .run();

    println!("Server running at http://{IP}:{PORT}");

    server.await
}
