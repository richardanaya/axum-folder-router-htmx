
use axum::Router;

fn maybe_fn<F>(f: F) -> Option<F> {
    Some(f)
}

pub fn build_router() -> Router {
    let mut router = Router::new();
mod root;
if let Some(f) = maybe_fn(root::get) { router = router.route("/", axum::routing::get(f)); }
if let Some(f) = maybe_fn(root::post) { router = router.route("/", axum::routing::post(f)); }
if let Some(f) = maybe_fn(root::put) { router = router.route("/", axum::routing::put(f)); }
if let Some(f) = maybe_fn(root::delete) { router = router.route("/", axum::routing::delete(f)); }
if let Some(f) = maybe_fn(root::patch) { router = router.route("/", axum::routing::patch(f)); }
    router
}
