use mongodb::{Client, ClientOptions, ThreadedClient};

use crate::mongodb::db::ThreadedDatabase;

pub fn connect_mongodb() -> std::sync::Arc<mongodb::db::DatabaseInner> {
    let opts = ClientOptions::with_unauthenticated_ssl(None, false);
    let uri = "mongodb://cluster0-shard-00-00-3uvx2.gcp.mongodb.net:27017,cluster0-shard-00-01-3uvx2.gcp.mongodb.net:27017,cluster0-shard-00-02-3uvx2.gcp.mongodb.net:27017/test?ssl=true&replicaSet=Cluster0-shard-0&authSource=admin&retryWrites=true&w=majority";
    let m = Client::with_uri_and_options(uri, opts).expect("");
    let mut db = m.db("admin");
    db.auth("admin", "admin").unwrap();
    db = m.db("PareshTODO");
    db
}
