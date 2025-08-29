use pg_query;

fn main() {
    let sql = "CREATE TABLE test_table (
        id int PRIMARY KEY,
        name text,
        full_name text GENERATED ALWAYS AS (upper(name)) STORED
    )";
    
    let result = pg_query::parse(sql).unwrap();
    
    // Check the AST to see what's being parsed
    for stmt in &result.protobuf.stmts {
        if let Some(stmt) = &stmt.stmt {
            if let Some(node) = &stmt.node {
                match node {
                    pg_query::protobuf::NodeEnum::CreateStmt(create_stmt) => {
                        println!("Found CREATE statement");
                        for col in &create_stmt.table_elts {
                            if let Some(col_node) = &col.node {
                                match col_node {
                                    pg_query::protobuf::NodeEnum::ColumnDef(col_def) => {
                                        println!("Column: {}", col_def.colname);
                                        println!("Generated field: '{}'", col_def.generated);
                                        println!("Generated field empty: {}", col_def.generated.is_empty());
                                        if let Some(raw_default) = &col_def.raw_default {
                                            println!("Has raw_default: yes");
                                        } else {
                                            println!("Has raw_default: no");
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    let functions = result.functions();
    println!("Functions found: {:?}", functions);
}