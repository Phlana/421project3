use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct Connect4Computer {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props { }

pub enum Msg {}


impl Component for Connect4Computer {
    type Message = Msg;
    type Properties = Props;

    fn create (props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Connect4Computer { props }
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
//            <script>{console.log("loogging");}</script>
              <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
              </div>
                 <div class="col-md-offset-4 col-md-8">
                  <form  ng-submit="Game()">
                    <div class="col-md-offset-3 col-md-8">

                      <input id="textbox1" type="text" placeholder="Your Name" ng-model="newGame.Player1Name"/>
                      <input id="startbutton" class="button" type="submit" value="Start Game"/>
                    </div>
                  </form>
                  <div class="post">
                    <br></br>
//                    <h4>New Game:  {{game.Player1Name}} Vs {{game.Player2Name}}</h4>
//                    <small>(Disc Colors: {{game.Player1Name}} - <b>Red</b>    and    {{game.Player2Name}} - <b>Yellow</b>)</small>
                    <br></br>
                  </div>
                    <canvas id="gameboard" height="480" width="640"></canvas>

                </div>
            </>

        }
    }
}