use start_axum::models::todo::Todo;
use surrealdb::opt::auth::Root;

#[cfg(feature = "ssr")]
#[tracing::instrument(level = "trace", fields(error), skip_all)]
pub async fn handle_server_fns_with_db(
    axum::extract::State(db): axum::extract::State<
        surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
    >,
    path: axum::extract::Path<String>,
    headers: http::HeaderMap,
    raw_query: axum::extract::RawQuery,
    req: http::Request<axum::body::Body>,
) -> impl axum::response::IntoResponse {
    use leptos::provide_context;
    leptos_axum::handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move |cx| provide_context(cx, db.clone()),
        req,
    )
    .await
}

#[cfg(feature = "ssr")]
#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    use axum::{extract::Extension, routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use start_axum::components::app::*;
    use start_axum::fileserv::file_and_error_handler;
    use std::sync::Arc;

    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::Surreal;

    let db = Surreal::new::<Ws>(dotenv!("SURREALDB_URL"))
        .await
        .expect("connect to db");

    let username = dotenv!("SURREALDB_USERNAME");
    let password = dotenv!("SURREALDB_PASSWORD");
    db.signin(Root { username, password })
        .await
        .expect("should be able to login");

    db.use_ns(dotenv!("SURREALDB_NS"))
        .use_db(dotenv!("SURREALDB_DATABASE"))
        .await
        .expect("change ns and db");

    Todo::register();

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    // build our application with a route
    let app = Router::new()
        //.route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/api/*fn_name", post(handle_server_fns_with_db))
        .with_state(db)
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
