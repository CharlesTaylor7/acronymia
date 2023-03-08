use cfg_if::cfg_if;
use std::sync::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use acronymia::{App, AppProps};
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};

        #[get("/api/events")]
        async fn server_events() -> impl Responder {
            use futures::StreamExt;

            let stream =
                futures::stream::once(async { acronymia::api::demo().await.unwrap_or(0) })
                    .map(|value| {
                        Ok(web::Bytes::from(format!(
                            "event: message\ndata: {value}\n\n"
                        ))) as Result<web::Bytes>
                    });
            HttpResponse::Ok()
                .insert_header(("Content-Type", "text/event-stream"))
                .streaming(stream)
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {

            let _ = acronymia::api::register_server_functions();

            // setting to `None` defaults to cargo-leptos & its env vars
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr.clone();

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(|cx| view! { cx, <App/> });

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
                    .service(server_events)
                    .route( "/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(
                        leptos_options.to_owned(),
                        routes.to_owned(),
                        |cx| view! { cx, <App/> } ,
                    )
                    .service(Files::new("/", site_root))
                    .wrap(middleware::Compress::default())
            })
            .bind(&addr)?
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
