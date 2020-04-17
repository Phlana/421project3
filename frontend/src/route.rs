//! All available routes within this application for fragment based routing

use yew_router::prelude::*;

#[derive(Clone, Switch)]
pub enum RouterTarget {
    #[to = "/#HowToConnect"]
    Error,

    #[to = "/#loading"]
    Loading,

    #[to = "/#login"]
    Login,

    #[to = "/#content"]
    Content,
}
