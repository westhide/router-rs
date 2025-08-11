use axum::{Router, routing::MethodRouter};

use crate::route::{Route, RouteItem};

pub trait RouterExt<S> {
    fn new() -> Self;

    fn route(self, route: RouteItem<&str, MethodRouter<S>>) -> Self;
}

impl<S> RouterExt<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn new() -> Self {
        Router::new()
    }

    fn route(self, route: RouteItem<&str, MethodRouter<S>>) -> Self {
        self.route(route.rule(), route.into_handler())
    }
}
