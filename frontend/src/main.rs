#![recursion_limit="512"]
use yew::{prelude::*, services::fetch::FetchTask, agent::Bridged, format::Json,html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

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

        if let Some(cc) = &self.child_component {
            match cc {
                RouterTarget::Error => {
                    html! {
                    <>
            //            <!-- Sidenav/menu -->
                        <div class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav",>
                            <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%",>{"Close Menu"}</a>
                            <div class="w3-container",>
                                <h3 class="w3-padding-64",>{"(lay Connect4"}<br/> {"/ TOOT-OTTO"}</h3>
                            </div>
                            <a href="/HowToConnect4" class="w3-padding w3-hover-white">{"How to Play Connect4"}</a>
                            <a href="#/Connect4Computer" class="w3-padding w3-hover-white">{"Play Connect4 With Computer"}</a>
                            <a href="#/Connect4Human" class="w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</a>
                            <br></br>
                            <a href="#/HowToToot" class="w3-padding w3-hover-white">{"How to Play TOOT-OTTO"}</a>
                            <a href="#/TootOttoComputer" class="w3-padding w3-hover-white">{"Play Toot-Otto With Computer"}</a>
                            <a href="#/TootOttoHuman" class="w3-padding w3-hover-white">{"Play Toot-Otto With Another Human"}</a>
                            <br></br>
                            <a href="#/ScoreBoard" class="w3-padding w3-hover-white">{"View Game History"}</a>
                            <a href="#/Scores" class="w3-padding w3-hover-white">{"Score Board"}</a>
                        </div>

            //            <!-- Top menu on small screens -->
                        <header class="w3-container w3-top w3-hide-large w3-red w3-xlarge w3-padding">
                            <a href="javascript:void(0)" class="w3-btn w3-red w3-border w3-border-white w3-margin-right">{"&#9776;"}</a>
                            <span>{"Connect 4 with MEAN"}</span>
                        </header>

            //            <!-- Overlay effect when opening sidenav on small screens -->
                        <div class="w3-overlay w3-hide-large" style="cursor:pointer" title="close side menu" id="myOverlay"></div>

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
}

fn main() {
    yew::start_app::<App>();
}
