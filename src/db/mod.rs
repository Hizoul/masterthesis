use crate::game_logic::field::{GameField};
use mongodb::{Client,ThreadedClient,doc,bson};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;

pub fn record_gamefield(field: GameField, players: Vec<String>) {
  let client = Client::connect("localhost", 27017).expect("Failed to initialize mongodb");
  let col = client.db("contetro").collection("rustylogs");
  let insert_players = bson::to_bson(&players).unwrap();
  let insert_log = bson::to_bson(&field.log).unwrap();
  col.insert_one(doc!{
    "log" => insert_log,
    "players" =>  insert_players
  }, None).ok().expect("Could not insert document");
}

pub struct DbConnector {
  col: Collection
}

impl DbConnector {
  pub fn new() -> DbConnector {
    let client = Client::connect("localhost", 27017).expect("Failed to initialize mongodb");
    let col = client.db("contetro").collection("rustylogs");
    DbConnector {
      col
    }
  }
  pub fn record_gamefield(&mut self, field: GameField, players: Vec<String>) {
    let insert_players = bson::to_bson(&players).unwrap();
    let insert_log = bson::to_bson(&field.log).unwrap();
    self.col.insert_one(doc!{
      "log" => insert_log,
      "players" =>  insert_players
    }, None).ok().expect("Could not insert document");
  }
}