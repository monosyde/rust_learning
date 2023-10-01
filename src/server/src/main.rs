use std::env;
use warp::Filter;
extern crate kaiten_api;

#[tokio::main]
async fn main() {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3030);

    let info_vec = kaiten_api::utils::get_info().await;
    let html_content: String = info_vec
            .iter()
            .map(|info| {
                let members_html: String = info.members
                    .iter()
                    .map(|member| format!("<li>{}</li>", member))
                    .collect();
                format!(
                    r#"
                    <div>
                        <h1>{}</h1>
                        <ul>{}</ul>
                    </div>
                    "#,
                    info.title, members_html
                )
            })
            .collect();

            
    let info = warp::path("info")
    .map(move || {
        warp::reply::html(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Info Page</title>
            </head>
            <body>
                {}
            </body>
            </html>
            "#,
            html_content
        ))
    })
    .with(warp::cors().allow_any_origin());

    let routes = info;
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
