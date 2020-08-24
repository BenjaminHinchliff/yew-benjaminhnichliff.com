use yew::prelude::*;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, Request, Response, FetchTask, FetchOptions};
use yew::format::Json;
use web_sys::RequestMode;
use serde::{Serialize, Deserialize};
use anyhow::Error;
use log::debug;

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    name: String,
    created_at: String,
}

#[derive(Debug)]
pub enum Msg {
    Repos(Vec<Repo>),
    Error(Error),
    UnknownError,
}

pub struct RepoList {
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

impl Component for RepoList {
    type Message = Msg;
    type Properties = ();
    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let get_request = Request::get("https://api.github.com/users/BenjaminHinchliff/repos?per_page=100")
            .body(Nothing)
            .expect("Failed to build request");
        
        let options = FetchOptions {
            mode: Some(RequestMode::Cors),
            ..FetchOptions::default()
        };
        
        let callback = link.callback(|response: Response<Json<Result<Vec<Repo>, Error>>>| {
            if let (meta, Json(Ok(body))) = response.into_parts() {
                if meta.status.is_success() {
                    return Msg::Repos(body);
                }
            }
            Msg::UnknownError
        });

        let task = FetchService::fetch_with_options(get_request, options, callback).unwrap();

        Self { link, task: Some(task) }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        debug!("{:?}", msg);
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "repo list rs" }</p>
        }
    }
}
