use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{prelude::*, switch::AllowMissing};

pub struct HowToToot {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props { }

pub enum Msg {}


impl Component for HowToToot {
    type Message = Msg;
    type Properties = Props;

    fn create (props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HowToToot { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        html !{
            <>
//            <!-- Sidenav/menu -->
                <div class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav",>
                    <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%",>{"Close Menu"}</a>
                    <div class="w3-container",>
                        <h3 class="w3-padding-64",>{"Play Connect4"}<br/> {"/ TOOT-OTTO"}</h3>
                    </div>
                    <a href="/#loading" class="w3-padding w3-hover-white">{"How to Play Connect4"}</a>
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
            </>

        }
    }
}