use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}

#[cfg(test)]
mod tests {
    #[async_std::test]
    async fn handle_create_post_testcase_01() {
        assert!(true);
    }
}
