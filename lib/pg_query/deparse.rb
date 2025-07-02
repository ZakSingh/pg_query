module PgQuery
  class ParserResult
    def deparse(pretty_print: false)
      PgQuery.deparse(@tree, pretty_print: pretty_print)
    end
  end

  # Reconstruct all of the parsed queries into their original form
  def self.deparse(tree, pretty_print: false)
    protobuf_encoded = if PgQuery::ParseResult.method(:encode).arity == 1
                         PgQuery::ParseResult.encode(tree)
                       elsif PgQuery::ParseResult.method(:encode).arity == -1
                         PgQuery::ParseResult.encode(tree, recursion_limit: 1_000)
                       else
                         raise ArgumentError, 'Unsupported protobuf Ruby API'
                       end

    if pretty_print
      PgQuery.deparse_protobuf_pretty_print(protobuf_encoded).force_encoding('UTF-8')
    else
      PgQuery.deparse_protobuf(protobuf_encoded).force_encoding('UTF-8')
    end
  end

  # Convenience method for deparsing a statement of a specific type
  def self.deparse_stmt(stmt)
    deparse(PgQuery::ParseResult.new(version: PG_VERSION_NUM, stmts: [PgQuery::RawStmt.new(stmt: PgQuery::Node.from(stmt))]))
  end

  # Convenience method for deparsing an expression
  def self.deparse_expr(expr)
    deparse_stmt(PgQuery::SelectStmt.new(where_clause: expr, op: :SETOP_NONE)).gsub('SELECT WHERE ', '')
  end
end
