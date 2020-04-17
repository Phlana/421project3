use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{prelude::*, switch::AllowMissing};

pub struct Landing {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props { }

pub enum Msg {}


impl Component for Landing {
    type Message = Msg;
    type Properties = Props;

    fn create (props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Landing { props }
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
                <form ng-submit="Game()">
                  <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red"><b>{"Welcome"}</b></h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                    <p>{"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}
                    </p>

                    <ul>

                        <li>{"Connect 4"}</li>

                        <li>{"TOOT-OTTO"}</li>


                    </ul>
                    <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
                  </div>
                </form>
            </>

        }
    }
}