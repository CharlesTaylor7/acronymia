use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use acronymia::components::app::App;
        use acronymia::server::{sync, ws};
        use actix_files::Files;
        use actix_web::{App, web, middleware, HttpServer};
        use leptos::{view, get_configuration};
        use leptos_actix::{generate_route_list, LeptosRoutes};

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            sync::spawn_state_thread();

            // setting to `None` defaults to cargo-leptos & its env vars
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(|| view! { <App/> });

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
                    .service(web::resource("/ws").route(web::get().to(ws::handle_ws_request)))
                    .route( "/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(
                        leptos_options.to_owned(),
                        routes.to_owned(),
                        || view! { <App/> } ,
                    )
                    .service(Files::new("/", site_root))
                    .wrap(middleware::Compress::default())
            })
            .bind(addr)?
            .run()
            .await
        }
    } else {
        pub fn main() {
            // no client-side main function
            // unless we want this to work with e.g., Trunk for pure client-side testing
            // see lib.rs for hydration function instead
        }
    }
}
