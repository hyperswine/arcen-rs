use arcen::common::{Component, Router}
use pages::index::Index

fn main(config: Config) -> Status {
    arcen.render(App)
}

fn App() -> Component {
    @arcen {
        Arcen[router: Router, index: Index] {}
    }
}
