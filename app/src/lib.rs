use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="app">
                <div id="intro">
                    <div id="header">
                        <h1>{ "Benjamin Hinchliff" }</h1>
                        <p id="blerb">
                            {normalize_whitespace("
                            A skilled self-taught programmer with professional development
                            experience; coding community leader, assisting at local elementary and
                            middle school classes and camps and running self-founded high school
                            coding club; long active with presiding and participating in many
                            neighborhood activities through BSA Scouts.
                            ")}
                        </p>
                    </div>
                    <img id="portrait" src="./assets/self.jpeg" />
                </div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn normalize_whitespace(string: &str) -> String {
    String::from(string).split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
