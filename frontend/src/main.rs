#![recursion_limit="512"]
use yew::{prelude::*, services::fetch::FetchTask, agent::Bridged, format::Json,html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};
mod sidebar;
use crate::{
    sidebar::Sidebar,
};

struct App {
//    clicked: bool,
//    onclick: Callback<ClickEvent>,
    child_component: Option<RouterTarget>,
    router_agent: Box<dyn Bridge<RouteAgent<()>>>,
}

//use crate::{
//    route::RouterTarget,
//};
use yew_router::prelude::*;

#[derive(Clone, Switch)]
pub enum RouterTarget {
    #[to = "/#error"]
    Error,

    #[to = "/#loading"]
    Loading,

//    #[to = "/#login"]
//    Login,
//
//    #[to = "/#content"]
//    Content,
}
//use yew::{agent::Bridged, format::Json, html, prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::{RouteAgent, RouteRequest::ChangeRoute},
    switch::Switch,
};

enum Message {
//    Click,
    Route(Route<()>),
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut router_agent = RouteAgent::bridge(link.callback(Message::Route));
        Self {
            child_component: Some(RouterTarget::Error),
            router_agent,
//            clicked: false,
//            onclick: link.callback(|_| Message::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Route to the appropriate child component
            Message::Route(route) => self.child_component = RouterTarget::switch(route),
//            Message::Click => {
//                self.clicked = true;
//                true // Indicate that the Component should re-render
//            }
//            // Route to the appropriate child component
//            Message::Route(route) => self.child_component = RouterTarget::switch(route),
        }
        true
    }

    fn view(&self) -> Html {
//        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };

        html !{
        <>
        <Sidebar/> {
            if let Some(cc) = &self.child_component {
                match cc {
                    RouterTarget::Error => {
                        html! {
                    <>
            //            <!-- !PAGE CONTENT! -->
                        <div class="w3-main" style="margin-left:390px;margin-right:40px">
                            <div class="w3-container" id="services" style="margin-top:75px">
                                <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play Connect 4"}</b></h5>
                                <hr style="width:50px;border:5px solid red" class="w3-round"/>
                                <p>{"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."} </p>
                                <br/>
                                <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
                                <ul>

                                    <li>{"A new game describes discs of which color belongs to which player"}</li>

                                    <li>{"Click on the desired column on the game board to place your disc"}</li>

                                    <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>

                                </ul>
                                <br/> {"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
                            </div>
                        </div>
                        </>
                    }
                    }
                    RouterTarget::Loading => {
                        html! { "ITS LOADING! "}
                    }
                }
            } else {
                html! { "No Child component"}
            }
        }
        </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
