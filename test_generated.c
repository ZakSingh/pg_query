SELECT pg_query_parse("CREATE TABLE t (a int, b int GENERATED ALWAYS AS (a + 1) STORED);");
int main() {}
