use pg_query;

fn main() {
    // Test basic GENERATED column
    let sql = "CREATE TABLE test_table (
        id int PRIMARY KEY,
        name text,
        full_name text GENERATED ALWAYS AS (upper(name)) STORED
    )";
    
    let result = pg_query::parse(sql).unwrap();
    let functions = result.functions();
    
    println!("SQL: {}", sql);
    println!("Functions found: {:?}", functions);
    
    // Should find the 'upper' function
    assert!(functions.contains(&"upper".to_string()));
    
    // Test complex GENERATED column with nested functions
    let sql2 = "CREATE TABLE test_table2 (
        id int PRIMARY KEY,
        created_at timestamp,
        data jsonb,
        processed_data jsonb GENERATED ALWAYS AS (
            jsonb_build_object('timestamp', now(), 'data', upper(data::text))
        ) STORED
    )";
    
    let result2 = pg_query::parse(sql2).unwrap();
    let functions2 = result2.functions();
    
    println!("\nSQL: {}", sql2);
    println!("Functions found: {:?}", functions2);
    
    // Should find nested functions
    assert!(functions2.contains(&"jsonb_build_object".to_string()));
    assert!(functions2.contains(&"now".to_string()));
    assert!(functions2.contains(&"upper".to_string()));
    
    println!("\nGENERATED column support is working correctly!");
}