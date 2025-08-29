#[cfg(test)]
use pg_query::parse;

#[test]
fn test_schema_qualified_function_names() {
    // Test CREATE FUNCTION with schema-qualified name
    let sql = r#"
        CREATE OR REPLACE FUNCTION schema_b.count_records()
        RETURNS INTEGER AS $$
        BEGIN
            RETURN 42;
        END;
        $$ LANGUAGE plpgsql;
    "#;
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.ddl_functions(), ["schema_b.count_records"]);
    assert_eq!(result.statement_types(), ["CreateFunctionStmt"]);
    
    // Test DROP FUNCTION with schema-qualified name
    let sql2 = "DROP FUNCTION schema_a.my_function();";
    let result2 = parse(sql2).unwrap();
    assert_eq!(result2.warnings.len(), 0);
    assert_eq!(result2.ddl_functions(), ["schema_a.my_function"]);
    assert_eq!(result2.statement_types(), ["DropStmt"]);
    
    // Test function call with schema-qualified name
    let sql3 = "SELECT schema_x.process_data(123);";
    let result3 = parse(sql3).unwrap();
    assert_eq!(result3.warnings.len(), 0);
    assert_eq!(result3.call_functions(), ["schema_x.process_data"]);
    assert_eq!(result3.statement_types(), ["SelectStmt"]);
    
    // Test multiple schema parts
    let sql4 = "CREATE FUNCTION catalog.schema.func() RETURNS void AS $$ BEGIN END; $$ LANGUAGE plpgsql;";
    let result4 = parse(sql4).unwrap();
    assert_eq!(result4.warnings.len(), 0);
    assert_eq!(result4.ddl_functions(), ["catalog.schema.func"]);
}