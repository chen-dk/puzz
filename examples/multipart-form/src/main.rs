use puzz::http::StatusCode;
use puzz::response::IntoResponse;
use puzz::service::ServiceExt;
use puzz::{extract, middleware, service_fn, BoxError, Request, Server};
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() {
    Server::new(|| {
        service_fn(form_data)
            .map_response(IntoResponse::into_response)
            .with(middleware::handle_error(|err: BoxError| {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }))
    })
    .bind(([127, 0, 0, 1], 80))
    .run()
    .await
    .unwrap();
}

async fn form_data(mut request: Request) -> Result<impl IntoResponse, BoxError> {
    let form = extract::multipart(&mut request)?;

    form.try_fold(String::new(), |mut text, field| async move {
        text.push_str(&format!("{}:{:?}\n", field.name(), field.filename()));
        Ok(text)
    })
    .await
    .map_err(From::from)
}
