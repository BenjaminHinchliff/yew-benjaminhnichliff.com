#![recursion_limit="1024"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    res: Res,
}

enum Msg {
}

#[wasm_bindgen]
#[derive(Clone, PartialEq)]
pub struct Res {
    portrait: String,
}

#[wasm_bindgen]
impl Res {
    #[wasm_bindgen(constructor)]
    pub fn new(portrait: String) -> Res {
        Res { portrait }
    }
}

#[derive(Properties, Clone, PartialEq)]
struct Props {
    res: Res,
}

impl Props {
    fn new(res: Res) -> Props {
        Props { res }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, res: props.res }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
                            { normalize_whitespace("
                            A skilled self-taught programmer with professional development
                            experience; coding community leader, assisting at local elementary and
                            middle school classes and camps and running self-founded high school
                            coding club; long active with presiding and participating in many
                            neighborhood activities through BSA Scouts.
                            ") }
                        </p>
                    </div>
                    <img id="portrait" src={ &self.res.portrait } />
                </div>
                <div id="content">
                    <h2 id="projects-header" class="text-center">{ "Projects" }</h2>
                    <p id="projects-subheader" class="text-center muted">
                        { "(more recent years)" }
                    </p>
                </div>
                <p id="copyright" class="muted text-center">
                    { "Copyright (&copy;) 2020 Benjamin Hinchliff" }
                </p>
            </div>
        }
    }
}

fn normalize_whitespace(string: &str) -> String {
    String::from(string).split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[wasm_bindgen]
pub fn run_app(res: Res) {
    App::<Model>::new().mount_to_body_with_props(Props::new(res));
}
