use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub created_at: String,
    pub html_url: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub name: String,
    pub created_at: String,
    pub html_url: String,
}

impl Component for Repo {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let Props { name, created_at, html_url } = props;
        Repo { name, created_at, html_url }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <a class="repo" href=self.html_url.clone() rel="noopener noreferrer" target="_blank">
                <h5 class="name">{ &self.name }</h5>
                <h6 class="date">{ &self.created_at }</h6>
                <p class="description">{ "(no description)" }</p>
            </a>
        }
    }
}
