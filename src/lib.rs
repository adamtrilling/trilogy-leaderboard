#[macro_use]
extern crate seed;

use futures::Future;

use seed::prelude::*;
use seed::{fetch, Request};
use serde::Deserialize;

// Model
#[derive(Clone, Debug, Deserialize)]
struct Leaderboard {
    data: LeaderboardData,
}

#[derive(Clone, Debug, Deserialize)]
struct LeaderboardData {
    runs: Vec<Run>,
}

#[derive(Clone, Debug, Deserialize)]
struct Run {
    run: RunData,
}

#[derive(Clone, Debug, Deserialize)]
struct RunData {
    players: Vec<Player>,
    times: TimeData,
}

#[derive(Clone, Debug, Deserialize)]
struct Player {
    id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct TimeData {
    primary_t: u32,
}

struct Model {
    z1: Option<Leaderboard>,
    z2: Option<Leaderboard>,
    z3: Option<Leaderboard>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            z1: None,
            z2: None,
            z3: None,
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
    let url = "https://www.speedrun.com/api/v1/leaderboards/369p0g1l/category/wdmw952q";
    seed::log!("fetching data from", url);
    Request::new(url.into()).fetch_json_data(Msg::DataFetched)
}

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    seed::log!("updating with", msg);
    match msg {
        Msg::FetchData => {
            orders.skip().perform_cmd(fetch_data());
        }
        Msg::DataFetched(Ok(leaderboard)) => model.z1 = Some(leaderboard),
        Msg::DataFetched(Err(_)) => {}
    }
}


// View

fn view(model: &Model) -> El<Msg> {
    button![
        simple_ev(Ev::Click, Msg::FetchData),
        format!("Hello, World × {:?}", 7)
    ]
}

#[wasm_bindgen]
pub fn render() {
    let app = seed::App::build(Model::default(), update, view)
        .finish()
        .run();

    app.update(Msg::FetchData);
}