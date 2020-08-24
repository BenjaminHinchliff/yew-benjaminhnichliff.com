#![recursion_limit="1024"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod utils;
use utils::normalize_whitespace;

mod logging;

mod repo;

mod repo_list;
use repo_list::RepoList;

struct Model {
    _link: ComponentLink<Self>,
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
        Self { _link: link, res: props.res }
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
                    <RepoList />
                </div>
                <p id="copyright" class="muted text-center">
                    { "Copyright (\u{00A9}) 2020 Benjamin Hinchliff" }
                </p>
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn run_app(res: Res) {
    logging::init_logging();
    logging::init_panic();
    App::<Model>::new().mount_to_body_with_props(Props::new(res));
}
