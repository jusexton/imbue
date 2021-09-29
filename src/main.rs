#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::imbue::{average_imbue, last_known_imbue, zeroed_imbue, DataPoint, ImbueContext};

mod imbue;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ImbueRequest {
    dataset: Vec<DataPoint>,
    strategy: ImbueStrategy
}

impl From<ImbueRequest> for ImbueContext {
    fn from(request: ImbueRequest) -> Self {
        ImbueContext::new(request.dataset)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ImbueResponse {
    dataset: Vec<DataPoint>,
}

impl ImbueResponse {
    fn new(dataset: Vec<DataPoint>) -> Self {
        ImbueResponse { dataset }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "snake_case")]
enum ImbueStrategy {
    Average,
    Zeroed,
    LastKnown,
}

#[post("/imbue", data = "<request>", format = "json")]
fn imbue_data(request: Json<ImbueRequest>) -> Json<ImbueResponse> {
    let imbue = match request.strategy {
        ImbueStrategy::Average => average_imbue,
        ImbueStrategy::Zeroed => zeroed_imbue,
        ImbueStrategy::LastKnown => last_known_imbue,
    };
    let context = &ImbueContext::from(request.0);
    let imbued_dataset = imbue(context);

    Json(ImbueResponse::new(imbued_dataset))
}

// Will need this later https://cprimozic.net/blog/rust-rocket-cloud-run/#deploying
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![imbue_data])
}
