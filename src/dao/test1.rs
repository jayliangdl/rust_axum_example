#[derive(Debug)]
struct Account {
    id: i32,
    name: String
}

#[cfg(test)]
#[tokio::test]
async fn ts() -> Result<(), sqlx::Error> {
    use crate::utils::db::init_pool;
    let pool: sqlx::Pool<sqlx::MySql> = init_pool().await.expect("Cannot init the database pool");
    // let mut conn = <impl sqlx::Executor>;

    let account = sqlx::query_as!(
        Account,
        "select * from (select (1) as id, 'Herp Derpinson' as name) accounts where name like ? and id=?",
        "%erp%",
        1i32
    )
    .fetch_all(&pool)
    .await?;

    println!("{account:?}");
    // println!("{}: {}", account.id, account.name);
    Ok(())
}
