use gluesql::{Glue, MemoryStorage};

fn main() {
    // Glue 데이터베이스 생성
    let storage = MemoryStorage::default();
    let mut glue = Glue::new(storage);

    // 테이블 생성
    let create_table_sql = "CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)";
    let result = glue.execute(create_table_sql);
    println!("Create Table Result: {:?}", result);

    // 데이터 삽입
    let insert_sql = "INSERT INTO users (id, name, age) VALUES (1, 'John Doe', 25)";
    let result = glue.execute(insert_sql);
    println!("Insert Result: {:?}", result);

    // 데이터 조회
    let select_sql = "SELECT * FROM users";
    let result = glue.execute(select_sql);
    println!("Select Result: {:?}", result);
}
