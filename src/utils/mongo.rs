use mongodb::{Client, ClientOptions, ThreadedClient};

use crate::mongodb::db::ThreadedDatabase;

pub fn connect_mongodb() -> std::sync::Arc<mongodb::db::DatabaseInner> {
    let opts = ClientOptions::with_unauthenticated_ssl(None, false);
    let uri = "mongodb://geoworker-shard-00-00-aad5x.mongodb.net:27017,geoworker-shard-00-01-aad5x.mongodb.net:27017,geoworker-shard-00-02-aad5x.mongodb.net:27017/test?replicaSet=GeoWorker-shard-0";
    let m = Client::with_uri_and_options(uri, opts).expect("");
    let mut db = m.db("admin");
    db.auth("GWAdmin", "GWPass").unwrap();
    db = m.db("GeoWorker");
    db
}