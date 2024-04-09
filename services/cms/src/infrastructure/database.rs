// use diesel::{Connection, PgConnection};
// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
//
// pub fn establish_connection(connection_string: &str) -> PgConnection {
//     PgConnection::establish(connection_string)
//         .expect(&format!("Error connecting to {}", connection_string))
// }
//
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
//
// pub fn migrate_database(connection_string: &str) {
//     let mut connection = establish_connection(connection_string);
//     connection.run_pending_migrations(MIGRATIONS).unwrap();
// }
