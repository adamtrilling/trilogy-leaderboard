#[macro_use]
extern crate seed;

pub mod model;

use futures::Future;
use futures::future::join_all;

use seed::prelude::*;
use seed::{fetch, Request};

use model::Leaderboard;

// Model

struct Model {
    leaderboards: Vec<Leaderboard>
}

impl Default for Model {
    fn default() -> Self {
        Self {
            leaderboards: Vec::new()
        }
    }
}


// Update

#[derive(Clone, Debug)]
enum Msg {
    FetchData,
    DataFetched(fetch::ResponseDataResult<Leaderboard>),
}

fn fetch_data() -> impl Future<Item = Msg, Error = Msg> {
    let urls = vec![
        "https://www.speedrun.com/api/v1/leaderboards/369p0g1l/category/wdmw952q",
        "https://www.speedrun.com/api/v1/leaderboards/pd0wk21e/category/9d86o62n",
        "https://www.speedrun.com/api/v1/leaderboards/9d3rr0dl/category/wk6jz5rd"
    ];
    let reqs = join_all(urls.iter().map(|&url| 
        Request::new(url.into()).fetch_json_data(Msg::DataFetched)
    ).collect());
}

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    seed::log!("updating with", msg);
    match msg {
        Msg::FetchData => {
            orders.skip().perform_cmd(fetch_data());
        }
        Msg::DataFetched(Ok(leaderboard)) => model.leaderboards.push(leaderboard),
        Msg::DataFetched(Err(_)) => {}
    }
}


// View

fn view(model: &Model) -> El<Msg> {
    button![
        simple_ev(Ev::Click, Msg::FetchData),
        format!("Hello, World × {:?}", model.leaderboards)
    ]
}

#[wasm_bindgen]
pub fn render() {
    let app = seed::App::build(Model::default(), update, view)
        .finish()
        .run();

    app.update(Msg::FetchData);
}