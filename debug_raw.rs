fn main() {
    // Try a simple GENERATED column
    let sql = "CREATE TABLE t (a int, b int GENERATED ALWAYS AS (a + 1) STORED);";
    
    println!("Testing SQL: {}", sql);
    
    match pg_query::parse(sql) {
        Ok(result) => {
            println!("Parse successful!");
            println!("Raw protobuf debug: {:#?}", result.protobuf);
            
            let functions = result.functions();
            println!("Functions extracted: {:?}", functions);
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
    
    // Also try with DEFAULT to confirm that works
    let sql2 = "CREATE TABLE t2 (a int, b int DEFAULT generate_series(1,10));";
    
    println!("\nTesting SQL: {}", sql2);
    
    match pg_query::parse(sql2) {
        Ok(result) => {
            println!("Parse successful!");
            let functions = result.functions();
            println!("Functions extracted: {:?}", functions);
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}