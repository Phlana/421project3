use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct TootOttoHuman {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props { }

pub enum Msg {}


impl Component for TootOttoHuman {
    type Message = Msg;
    type Properties = Props;

    fn create (props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TootOttoHuman { props }
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
              <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
              </div>
                <div class="col-md-offset-4 col-md-8">
                  <form  ng-submit="Game()">

                    <div class="col-md-offset-3 col-md-8">
                      <input id="textbox1" type="text" placeholder="Player 1's Name" ng-model="newGame.Player1Name"/>
                      <input id="textbox2" type="text" placeholder="Player 2's Name" ng-model="newGame.Player2Name"/>
                      <input id="startbutton" class="button" type="submit" value="Start Game"/>
                    </div>
                  </form>
                  <div class="post" ng-repeat="game in games">
                    <br></br>
                    <h4>{"New Game:  {{game.Player1Name}} Vs {{game.Player2Name}}"}</h4>
//                    <small>(Winning Combination: {{game.Player1Name}} - <b>TOOT</b>    and    {{game.Player2Name}} - <b>OTTO</b>)</small>
                    <br></br>

//                    <form>
//                      <h4>Select a Disc Type   :
//                        <input type="radio" name="choice" value="T" checked ng-model="newGame.Label"> T
//                        <input type="radio" name="choice" value="O" ng-model="newGame.Label"> O
//                  </form>
//                 </h4>
                  </div>
                    <canvas id="gameboard" height="480" width="640"></canvas>

                </div>
            </>

        }
    }
}