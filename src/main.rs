#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use acronymia::components::app::App;
    use acronymia::server::{sync, ws};
    use actix_files::Files;
    use actix_web::{middleware, web, App, HttpServer};
    use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    use leptos::get_configuration;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    sync::spawn_state_thread();

    // setting to `None` defaults to cargo-leptos & its env vars
    let conf = get_configuration(None).await.unwrap();

    let addr = conf.leptos_options.site_addr;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        let routes = &routes;

        App::new()
            .service(web::resource("/ws").route(web::get().to(ws::handle_ws_request)))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .service(Files::new("/", site_root))
            .wrap(middleware::Compress::default())
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
    })
    .bind(addr)?
    .run()
    .await
}

/// no client-side main function
/// unless we want this to work with e.g., Trunk for pure client-side testing
/// see lib.rs for hydration function instead
#[cfg(feature = "hydrate")]
pub fn main() {}
