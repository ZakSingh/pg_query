use pg_query;

#[test]
fn test_generated_column_basic() {
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
    
    // The GENERATED column test should now work
    assert!(functions.contains(&"upper".to_string()));
}

#[test]
fn test_generated_column_complex() {
    // Test complex GENERATED column with nested functions
    let sql = "CREATE TABLE test_table2 (
        id int PRIMARY KEY,
        created_at timestamp,
        data jsonb,
        processed_data jsonb GENERATED ALWAYS AS (
            jsonb_build_object('timestamp', now(), 'data', upper(data::text))
        ) STORED
    )";
    
    let result = pg_query::parse(sql).unwrap();
    let functions = result.functions();
    
    println!("SQL: {}", sql);
    println!("Functions found: {:?}", functions);
    
    // Should find nested functions
    assert!(functions.contains(&"jsonb_build_object".to_string()));
    assert!(functions.contains(&"now".to_string()));
    assert!(functions.contains(&"upper".to_string()));
}

#[test]
fn test_generated_column_with_default() {
    // Test table with both DEFAULT and GENERATED columns
    let sql = "CREATE TABLE test_table3 (
        id int PRIMARY KEY,
        name text DEFAULT 'unnamed',
        created_at timestamp DEFAULT now(),
        full_name text GENERATED ALWAYS AS (concat(name, '_processed')) STORED
    )";
    
    let result = pg_query::parse(sql).unwrap();
    let functions = result.functions();
    
    println!("SQL: {}", sql);
    println!("Functions found: {:?}", functions);
    
    // Should find functions from both DEFAULT and GENERATED columns
    assert!(functions.contains(&"now".to_string()));
    assert!(functions.contains(&"concat".to_string()));
}