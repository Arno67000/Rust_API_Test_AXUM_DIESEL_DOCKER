use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use colored::Colorize;

use super::database::DB;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration(db: &DB) {
    let conn = &mut db.db_connect();

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(r) => println!(
            "{}{:?}",
            "Migrations succesfull : ".green().italic(),
            r
        ),
        Err(e) => eprintln!(
            "{}{:?}",
            "Migrations failed : ".red().bold(),
            e
        )
    };
}