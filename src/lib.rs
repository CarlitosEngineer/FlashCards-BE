pub use sea_orm_migration::prelude::*;

mod m2025_08_11_000001_create_countries;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m2025_08_11_000001_create_countries::Migration)]
    }
}
