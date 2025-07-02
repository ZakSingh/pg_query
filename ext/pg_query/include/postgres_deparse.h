#ifndef POSTGRES_DEPARSE_H
#define POSTGRES_DEPARSE_H

#include <stdbool.h>
#include <sys/types.h>

typedef struct PostgresDeparseComment {
    int after_location;          // Insert comment after the first node with this location
    bool newline_before_comment; // Insert a newline before inserting the comment (set if the source comment was separated from the prior token by at least one newline)
    bool newline_after_comment;  // Insert a newline after inserting the comment (set if the source comment was separated from the next token by at least one newline)
    char *str;                   // The actual comment string, including comment start/end tokens, and newline characters in comment (if any)
} PostgresDeparseComment;

typedef struct PostgresDeparseOpts {
    PostgresDeparseComment **comments;
    size_t comment_count;

    // Pretty print options
    bool pretty_print;
    int indent_size;             // Indentation size (Default 4 spaces)
    //bool commas_start_of_line; // Place separating commas at start of line
} PostgresDeparseOpts;

/* Forward declarations to allow using PostgresDeparseOpts without needing Postgres includes */
struct StringInfoData;
typedef struct StringInfoData *StringInfo;
struct RawStmt;

extern void deparseRawStmt(StringInfo str, struct RawStmt *raw_stmt);
extern void deparseRawStmtOpts(StringInfo str, struct RawStmt *raw_stmt, PostgresDeparseOpts opts);

#endif
