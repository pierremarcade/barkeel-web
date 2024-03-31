use axum::{
    body::{Bytes, Full},
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{fs, path::Path};
use tokio::fs::File;
use tokio_stream::StreamExt;

async fn image_middleware(req: Request<Full>) -> impl IntoResponse {
    let path = req.uri().path();
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() != 3 {
        return Response::builder().status(StatusCode::BAD_REQUEST).body(Bytes::from_static(b"Bad request")).unwrap();
    }

    let format = parts[1];
    let filename = parts[2];
    let image_path = format!("images/{}", filename);
    let resized_image_path = format!("images/{}/{}", format, filename);

    if Path::new(&resized_image_path).exists() {
        // L'image redimensionnée existe déjà, on la retourne
        let file = File::open(resized_image_path).await.unwrap();
        let stream = tokio_util::io::ReaderStream::new(file);
        return Response::new(Body::wrap_stream(stream));
    }

    // L'image redimensionnée n'existe pas, on la génère
    let image = image::open(image_path).unwrap().resize(400, 200, image::imageops::FilterType::Nearest);
    let mut buffer = Vec::new();
    image.write_to(&mut buffer, image::ImageOutputFormat::Jpeg).unwrap();

    // Sauvegarde de l'image redimensionnée
    let dir = format!("images/{}/", format);
    fs::create_dir_all(dir).unwrap();
    tokio::fs::write(resized_image_path, buffer).await.unwrap();

    // Retour de l'image redimensionnée
    Response::new(Body::from(buffer))
}
