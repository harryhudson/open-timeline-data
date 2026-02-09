// SPDX-License-Identifier: GPL-3.0-or-later

//!
//! Validate the entities & timelines in `/open-timeline-data/data/`.
//!

use open_timeline_crud::{restore, setup_database_at_path};
use sqlx::sqlite::SqlitePoolOptions;
use std::path::PathBuf;
use tempdir::TempDir;

#[tokio::main]
async fn main() {
    println!("Running validator");

    // Create database
    let tmp_dir = TempDir::new("open-timeline-data").unwrap();
    let file_path = tmp_dir.path().join("timeline.sqlite");
    setup_database_at_path(&file_path).await.unwrap();

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", file_path.display()))
        .await
        .unwrap();

    // Start a transaction
    let mut transaction = pool.begin().await.unwrap();

    // Paths to data
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data");
    let manual_dir = data_dir.join("manual");
    let wikidata_dir = data_dir.join("wikidata/data");

    // Attempt to restore the database (i.e. check the validity of the data)
    restore(&mut transaction, wikidata_dir).await.unwrap();
    restore(&mut transaction, manual_dir).await.unwrap();

    // Finish
    println!("Success");
}
