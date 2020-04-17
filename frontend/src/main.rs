#![recursion_limit="512"]
use yew::{prelude::*, agent::Bridged, html, Component, ComponentLink, Html, ShouldRender};
mod sidebar;
mod connect4computer;
mod connect4human;
mod howToConnect4;
mod howToToot;
mod landing;
mod scoreboard;
mod scores;
mod tootOttoComputer;
mod tootOttoHuman;

use crate::{
    sidebar::Sidebar,
    connect4computer::Connect4Computer,
    connect4human::Connect4Human,
    howToConnect4::HowToConnect4,
    howToToot::HowToToot,
    landing::Landing,
    scoreboard::Scoreboard,
    scores::Scores,
    tootOttoComputer::TootOttoComputer,
    tootOttoHuman::TootOttoHuman,
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
    #[to = "/#landing"]
    Landing,

    #[to = "/#connect4computer"]
    Connect4Computer,

    #[to = "/#connect4human"]
    Connect4Human,

    #[to = "/#howtoconnect4"]
    HowToConnect4,

    #[to = "/#howtotoot"]
    HowToToot,

    #[to = "/#scoreboard"]
    Scoreboard,

    #[to = "/#scores"]
    Scores,

    #[to = "/#tootottocomputer"]
    TootOttoComputer,

    #[to = "/#tootOttohuman"]
    TootOttoHuman,
}
//use yew::{agent::Bridged, format::Json, html, prelude::*, services::fetch::FetchTask};
use yew_router::{
    agent::{RouteAgent},
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
        let router_agent = RouteAgent::bridge(link.callback(Message::Route));
        Self {
            child_component: Some(RouterTarget::Landing),
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
        <Sidebar/>
        <div class="w3-main" style="margin-left:390px;margin-right:40px">
        {
            if let Some(cc) = &self.child_component {
                match cc {
                    RouterTarget::Landing => {
                        html! {
                            <Landing:/>
                        }
                    }
                    RouterTarget::Connect4Computer => {
                        html! {
                            <Connect4Computer:/>
                        }
                    }
                    RouterTarget::Connect4Human => {
                        html! {
                            <Connect4Human:/>
                        }
                    }
                    RouterTarget::HowToConnect4 => {
                        html! {
                            <HowToConnect4:/>
                        }
                    }
                    RouterTarget::HowToToot => {
                        html! {
                            <HowToToot:/>
                        }
                    }
                    RouterTarget::Scoreboard => {
                        html! {
                            <Scoreboard:/>
                        }
                    }
                    RouterTarget::Scores => {
                        html! {
                            <Scores:/>
                        }
                    }
                    RouterTarget::TootOttoComputer => {
                        html! {
                            <TootOttoComputer:/>
                        }
                    }
                    RouterTarget::TootOttoHuman => {
                        html! {
                            <TootOttoHuman:/>
                        }
                    }
                }
            } else {
                html! { "No Child component"}
            }
        }
        </div>
        </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
