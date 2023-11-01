use crate::config::Config;

pub async fn dev(config: Config) {
    let path = config.dest;
    let _port = std::env::args()
        .nth(2)
        .unwrap_or("7878".into())
        .parse::<u16>()
        .unwrap_or(7878_u16);
    let api = warp::fs::dir(path);
    let server = warp::serve(api);
    println!("server as http://127.0.0.1:7878 ");
    server.run(([0, 0, 0, 0], 7878)).await;
    
}
pub fn run_start() {}
