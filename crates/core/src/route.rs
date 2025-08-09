pub trait Route {
    type Rule;
    type Handler;

    fn rule(&self) -> &Self::Rule;

    fn handler(&self) -> &Self::Handler;

    fn into_handler(self) -> Self::Handler;
}

pub struct RouteItem<R, H> {
    rule: R,
    handler: H,
}

impl<R, H> RouteItem<R, H> {
    pub const fn new(rule: R, handler: H) -> Self {
        Self { rule, handler }
    }
}

impl<R, H> Route for RouteItem<R, H> {
    type Handler = H;
    type Rule = R;

    fn rule(&self) -> &Self::Rule {
        &self.rule
    }

    fn handler(&self) -> &Self::Handler {
        &self.handler
    }

    fn into_handler(self) -> Self::Handler {
        self.handler
    }
}
