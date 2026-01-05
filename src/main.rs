#![allow(unused_imports)]

use poly_rc::public::{
    PubClient, TokenId,
    events::Events,
    events::models::EventDTO,
    orderbook::OrderBook,
    pricing::{Pricing, models::MarketPriceDTO},
    sports::{Sports, models::SportsTeamsDTO},
};

use poly_rc::public::events;
use poly_rc::shared::Side;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let pub_client = PubClient::new();
    let result = pub_client
        .get_sports_teams(SportsTeamsDTO {
            limit: Some(1),
            offset: Some(0),
            order: Some("name".to_string()),
            ascending: Some(true),
            league: None,
            name: None,
            abbreviation: None,
        })
        .await;

    match result {
        Ok(res) => {
            println!("found orderbook summaries: {:?}", res);
        }
        Err(err) => {
            println!("error: could not get midpoint price");
            println!("error: {:?}", err);
        }
    }

    // let events_result = pub_client.get_events(EventDTO {
    //     active: None,
    //     closed: Some(true),
    //     limit: None,
    //     ..Default::default()
    // }).await;

    // match events_result {
    //     Ok(res) => {
    //         println!("found {} Events", res.len());

    //         dbg!(res);

    //     }
    //     Err(err) => {
    //         println!("error: could not get Events");
    //         println!("error: {:?}", err);
    //     }
    // }

    // let event_result = pub_client.get_event(
    //     String::from("2895"),
    //     EventDTO {
    //         active: None,
    //         closed: Some(true),
    //         limit: None,
    //         ..Default::default()
    //     }
    // ).await;

    // match event_result {
    //     Ok(res) => {
    //         println!("✅✅✅✅✅✅");
    //         dbg!(res);

    //     }
    //     Err(err) => {
    //         println!("error: could not get Event");
    //         println!("error: {:?}", err);
    //     }
    // }

    let event_tags = pub_client.get_event_tags(String::from("2895")).await;

    match event_tags {
        Ok(res) => {
            println!("✅✅✅✅✅✅");
            dbg!(res);
        }
        Err(err) => {
            println!("error: could not get Event Tag");
            println!("error: {:?}", err);
        }
    }

    // let event = pub_client.get_event_by_slug(
    //     String::from("nfl-will-the-jaguars-beat-the-texans-by-more-than-3pt5-points-in-their-december-19-matchup")
    // ).await;

    // match event {
    //     Ok(res) => {
    //         println!("✅✅✅✅✅✅");
    //         dbg!(res);

    //     }
    //     Err(err) => {
    //         println!("error: could not get Event");
    //         println!("error: {:?}", err);
    //     }
    // }
}
