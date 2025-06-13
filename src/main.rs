mod api;
pub mod timer;
use api::API;

fn main() {
    if let Some(mut api) = API::new("/tmp/sherlock-pomorodo.sock") {
        api.listen();
    }
}
