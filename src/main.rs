mod backend;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    backend::serve().await
}

// no main function if we're not using ssr feature
#[cfg(not(feature = "ssr"))]
fn main() {}
