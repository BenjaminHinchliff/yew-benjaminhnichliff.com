use yew::prelude::*;
use yew::format::Nothing;
use yew::services::fetch::{FetchService, Request, Response, FetchTask, FetchOptions};
use yew::format::Json;
use web_sys::RequestMode;
use anyhow::Error;

use super::repo::Repo;

#[derive(Debug)]
pub enum Msg {
    Repos(Vec<Repo>),
    RequestError,
}

pub struct Repos {
    _link: ComponentLink<Self>,
    task: Option<FetchTask>,
    repos: Vec<Repo>,
    errored: bool,
}

impl Component for Repos {
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
            Msg::RequestError
        });

        let task = FetchService::fetch_with_options(get_request, options, callback).unwrap();

        Self { _link: link, task: Some(task), repos: Vec::new(), errored: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Repos(repos) => {
                self.repos = repos;
                self.task = None;
                true
            },
            Msg::RequestError => {
                self.errored = true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.errored {
            html! {
                <p class="error text-center">{ "Something went wrong while trying to load projects" }</p> 
            }
        } else if self.task.is_some() {
            html! {
                <div class="loader" />
            }
        } else {
            html! {
                <div class="repos">
                    { for self.repos.iter().map(|r| r.render()) }
                </div>
            }
        }
    }
}
