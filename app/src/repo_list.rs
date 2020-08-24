use yew::prelude::*;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, Request, Response, FetchTask, FetchOptions};
use yew::format::Json;
use web_sys::RequestMode;
use anyhow::Error;
use log::debug;

use crate::repo::Repo;

#[derive(Debug)]
pub enum Msg {
    Repos(Vec<Repo>),
    RequestError(String),
}

pub struct RepoList {
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    repos: Vec<Repo>,
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
                    Msg::Repos(body)
                } else {
                    Msg::RequestError(format!("{}: {:?}", meta.status, body))
                }
            } else {
                Msg::RequestError("invalid response".to_owned())
            }
        });

        let task = FetchService::fetch_with_options(get_request, options, callback).unwrap();

        Self { link, task: Some(task), repos: Vec::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Repos(repos) => {
                self.repos = repos;
                self.task = None;
                true
            },
            _ => unimplemented!(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="repos">
                { for self.repos.iter().map(|r| r.render()) }
            </div>
        }
    }
}
