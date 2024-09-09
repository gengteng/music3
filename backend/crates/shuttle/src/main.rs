use music3_server::conf::Config;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(music3_server::route::router(Config::default())?.into())
}
