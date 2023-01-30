[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 71,
                            end: 75,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 96,
                                                    end: 105,
                                                    as_str(): "my_struct",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 108,
                                                                        end: 116,
                                                                        as_str(): "MyStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 108,
                                                                end: 116,
                                                                as_str(): "MyStruct",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 127,
                                                                        end: 128,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 130,
                                                                        end: 131,
                                                                        as_str(): "5",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 127,
                                                                    end: 131,
                                                                    as_str(): "a: 5",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 108,
                                                    end: 138,
                                                    as_str(): "MyStruct {\n        a: 5,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 92,
                                    end: 139,
                                    as_str(): "let my_struct = MyStruct {\n        a: 5,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 148,
                                                    end: 155,
                                                    as_str(): "my_enum",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 158,
                                                                                end: 164,
                                                                                as_str(): "MyEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 158,
                                                                            end: 164,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 166,
                                                                            end: 172,
                                                                            as_str(): "Number",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 158,
                                                                end: 172,
                                                                as_str(): "MyEnum::Number",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        10,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 173,
                                                                    end: 175,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 158,
                                                    end: 176,
                                                    as_str(): "MyEnum::Number(10)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 144,
                                    end: 177,
                                    as_str(): "let my_enum = MyEnum::Number(10);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 186,
                                                    end: 205,
                                                    as_str(): "my_struct_with_enum",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 208,
                                                                        end: 224,
                                                                        as_str(): "MyStructWithEnum",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 208,
                                                                end: 224,
                                                                as_str(): "MyStructWithEnum",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 235,
                                                                        end: 236,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 238,
                                                                                end: 247,
                                                                                as_str(): "my_struct",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 238,
                                                                        end: 247,
                                                                        as_str(): "my_struct",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 235,
                                                                    end: 247,
                                                                    as_str(): "a: my_struct",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 257,
                                                                        end: 258,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 260,
                                                                                end: 267,
                                                                                as_str(): "my_enum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 260,
                                                                        end: 267,
                                                                        as_str(): "my_enum",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 257,
                                                                    end: 267,
                                                                    as_str(): "b: my_enum",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 208,
                                                    end: 274,
                                                    as_str(): "MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 182,
                                    end: 275,
                                    as_str(): "let my_struct_with_enum = MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 284,
                                                    end: 285,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    String(
                                                        Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 289,
                                                            end: 294,
                                                            as_str(): "abcde",
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 288,
                                                    end: 295,
                                                    as_str(): "\"abcde\"",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 280,
                                    end: 296,
                                    as_str(): "let d = \"abcde\";",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 305,
                                                    end: 306,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 309,
                                                    end: 313,
                                                    as_str(): "true",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 301,
                                    end: 314,
                                    as_str(): "let e = true;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 323,
                                                    end: 324,
                                                    as_str(): "f",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        15,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 329,
                                                    as_str(): "15",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 319,
                                    end: 330,
                                    as_str(): "let f = 15;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 339,
                                                    end: 340,
                                                    as_str(): "g",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    U64(
                                                        170,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 343,
                                                    end: 353,
                                                    as_str(): "0b10101010",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 335,
                                    end: 354,
                                    as_str(): "let g = 0b10101010;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 363,
                                                    end: 364,
                                                    as_str(): "h",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: B256,
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 366,
                                                    end: 370,
                                                    as_str(): "b256",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    B256(
                                                        [
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                            170,
                                                        ],
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 373,
                                                    end: 631,
                                                    as_str(): "0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 359,
                                    end: 632,
                                    as_str(): "let h: b256 = 0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 638,
                                                                end: 648,
                                                                as_str(): "eight_args",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 638,
                                                        end: 648,
                                                        as_str(): "eight_args",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 649,
                                                                    end: 658,
                                                                    as_str(): "my_struct",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 649,
                                                            end: 658,
                                                            as_str(): "my_struct",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 660,
                                                                    end: 667,
                                                                    as_str(): "my_enum",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 660,
                                                            end: 667,
                                                            as_str(): "my_enum",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 669,
                                                                    end: 688,
                                                                    as_str(): "my_struct_with_enum",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 669,
                                                            end: 688,
                                                            as_str(): "my_struct_with_enum",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 690,
                                                                    end: 691,
                                                                    as_str(): "d",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 690,
                                                            end: 691,
                                                            as_str(): "d",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 693,
                                                                    end: 694,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 693,
                                                            end: 694,
                                                            as_str(): "e",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 696,
                                                                    end: 697,
                                                                    as_str(): "f",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 696,
                                                            end: 697,
                                                            as_str(): "f",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 699,
                                                                    end: 700,
                                                                    as_str(): "g",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 699,
                                                            end: 700,
                                                            as_str(): "g",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 702,
                                                                    end: 703,
                                                                    as_str(): "h",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 702,
                                                            end: 703,
                                                            as_str(): "h",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 638,
                                            end: 704,
                                            as_str(): "eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 638,
                                    end: 704,
                                    as_str(): "eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 744,
                                                    end: 751,
                                                    as_str(): "ls_than",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromTrait {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 756,
                                                                                end: 757,
                                                                                as_str(): "<",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 756,
                                                                                end: 757,
                                                                                as_str(): "<",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "lt",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 756,
                                                                            end: 757,
                                                                            as_str(): "<",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 756,
                                                                end: 757,
                                                                as_str(): "<",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        4,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 754,
                                                                    end: 755,
                                                                    as_str(): "4",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        5,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 758,
                                                                    end: 759,
                                                                    as_str(): "5",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 754,
                                                    end: 759,
                                                    as_str(): "4 < 5",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 740,
                                    end: 760,
                                    as_str(): "let ls_than = 4 < 5;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 769,
                                                    end: 776,
                                                    as_str(): "gt_than",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromTrait {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 781,
                                                                                end: 782,
                                                                                as_str(): ">",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 781,
                                                                                end: 782,
                                                                                as_str(): ">",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "gt",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 781,
                                                                            end: 782,
                                                                            as_str(): ">",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 781,
                                                                end: 782,
                                                                as_str(): ">",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        5,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 779,
                                                                    end: 780,
                                                                    as_str(): "5",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        4,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 783,
                                                                    end: 784,
                                                                    as_str(): "4",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 779,
                                                    end: 784,
                                                    as_str(): "5 > 4",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 765,
                                    end: 785,
                                    as_str(): "let gt_than = 5 > 4;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 794,
                                                    end: 796,
                                                    as_str(): "le",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromTrait {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 801,
                                                                                end: 803,
                                                                                as_str(): "<=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 801,
                                                                                end: 803,
                                                                                as_str(): "<=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "le",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 801,
                                                                            end: 803,
                                                                            as_str(): "<=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 801,
                                                                end: 803,
                                                                as_str(): "<=",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        4,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 799,
                                                                    end: 800,
                                                                    as_str(): "4",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        4,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 804,
                                                                    end: 805,
                                                                    as_str(): "4",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 799,
                                                    end: 805,
                                                    as_str(): "4 <= 4",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 790,
                                    end: 806,
                                    as_str(): "let le = 4 <= 4;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 815,
                                                    end: 817,
                                                    as_str(): "ge",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromTrait {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 822,
                                                                                end: 824,
                                                                                as_str(): ">=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 822,
                                                                                end: 824,
                                                                                as_str(): ">=",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ge",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 822,
                                                                            end: 824,
                                                                            as_str(): ">=",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 822,
                                                                end: 824,
                                                                as_str(): ">=",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        4,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 820,
                                                                    end: 821,
                                                                    as_str(): "4",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        4,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 825,
                                                                    end: 826,
                                                                    as_str(): "4",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 820,
                                                    end: 826,
                                                    as_str(): "4 >= 4",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 811,
                                    end: 827,
                                    as_str(): "let ge = 4 >= 4;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 836,
                                                    end: 838,
                                                    as_str(): "eq",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromTrait {
                                                                call_path: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "core",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 843,
                                                                                end: 845,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 843,
                                                                                end: 845,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "eq",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 843,
                                                                            end: 845,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 843,
                                                                end: 845,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        5,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 841,
                                                                    end: 842,
                                                                    as_str(): "5",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        5,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 846,
                                                                    end: 847,
                                                                    as_str(): "5",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 841,
                                                    end: 847,
                                                    as_str(): "5 == 5",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 832,
                                    end: 848,
                                    as_str(): "let eq = 5 == 5;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 861,
                                                    end: 865,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 854,
                                            end: 865,
                                            as_str(): "return true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 854,
                                    end: 865,
                                    as_str(): "return true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 86,
                            end: 868,
                            as_str(): "{\n    let my_struct = MyStruct {\n        a: 5,\n    };\n    let my_enum = MyEnum::Number(10);\n    let my_struct_with_enum = MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    };\n    let d = \"abcde\";\n    let e = true;\n    let f = 15;\n    let g = 0b10101010;\n    let h: b256 = 0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010;\n\n    eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h);\n\n    // test some comparisons\n    let ls_than = 4 < 5;\n    let gt_than = 5 > 4;\n    let le = 4 <= 4;\n    let ge = 4 >= 4;\n    let eq = 5 == 5;\n\n    return true;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb13d3811d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                        ),
                        start: 68,
                        end: 868,
                        as_str(): "fn main() -> bool {\n    let my_struct = MyStruct {\n        a: 5,\n    };\n    let my_enum = MyEnum::Number(10);\n    let my_struct_with_enum = MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    };\n    let d = \"abcde\";\n    let e = true;\n    let f = 15;\n    let g = 0b10101010;\n    let h: b256 = 0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010;\n\n    eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h);\n\n    // test some comparisons\n    let ls_than = 4 < 5;\n    let gt_than = 5 > 4;\n    let le = 4 <= 4;\n    let ge = 4 >= 4;\n    let eq = 5 == 5;\n\n    return true;\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb13d3811d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                        ),
                        start: 81,
                        end: 85,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 68,
            end: 868,
            as_str(): "fn main() -> bool {\n    let my_struct = MyStruct {\n        a: 5,\n    };\n    let my_enum = MyEnum::Number(10);\n    let my_struct_with_enum = MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    };\n    let d = \"abcde\";\n    let e = true;\n    let f = 15;\n    let g = 0b10101010;\n    let h: b256 = 0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010;\n\n    eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h);\n\n    // test some comparisons\n    let ls_than = 4 < 5;\n    let gt_than = 5 > 4;\n    let le = 4 <= 4;\n    let ge = 4 >= 4;\n    let eq = 5 == 5;\n\n    return true;\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 876,
                            end: 884,
                            as_str(): "MyStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 891,
                                    end: 892,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 891,
                                end: 897,
                                as_str(): "a: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 894,
                                end: 897,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb13d3811d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                        ),
                        start: 869,
                        end: 900,
                        as_str(): "struct MyStruct {\n    a: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 869,
            end: 900,
            as_str(): "struct MyStruct {\n    a: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 907,
                            end: 913,
                            as_str(): "MyEnum",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 920,
                                    end: 926,
                                    as_str(): "Number",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 928,
                                end: 931,
                                as_str(): "u64",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 920,
                                end: 931,
                                as_str(): "Number: u64",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 937,
                                    end: 941,
                                    as_str(): "Unit",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 943,
                                end: 945,
                                as_str(): "()",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 937,
                                end: 945,
                                as_str(): "Unit: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb13d3811d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                        ),
                        start: 902,
                        end: 948,
                        as_str(): "enum MyEnum {\n    Number: u64,\n    Unit: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 902,
            end: 948,
            as_str(): "enum MyEnum {\n    Number: u64,\n    Unit: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 957,
                            end: 973,
                            as_str(): "MyStructWithEnum",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 980,
                                    end: 981,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 983,
                                        end: 991,
                                        as_str(): "MyStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 980,
                                end: 991,
                                as_str(): "a: MyStruct",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 983,
                                end: 991,
                                as_str(): "MyStruct",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 997,
                                    end: 998,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 1000,
                                        end: 1006,
                                        as_str(): "MyEnum",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 997,
                                end: 1006,
                                as_str(): "b: MyEnum",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1000,
                                end: 1006,
                                as_str(): "MyEnum",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb13d3811d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                        ),
                        start: 950,
                        end: 1009,
                        as_str(): "struct MyStructWithEnum {\n    a: MyStruct,\n    b: MyEnum,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 950,
            end: 1009,
            as_str(): "struct MyStructWithEnum {\n    a: MyStruct,\n    b: MyEnum,\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 1014,
                            end: 1024,
                            as_str(): "eight_args",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Tuple(
                                                    [],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 1120,
                                                    end: 1126,
                                                    as_str(): "return",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1120,
                                            end: 1126,
                                            as_str(): "return",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1120,
                                    end: 1126,
                                    as_str(): "return",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 1114,
                            end: 1129,
                            as_str(): "{\n    return;\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1025,
                                    end: 1026,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 1028,
                                        end: 1036,
                                        as_str(): "MyStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1028,
                                end: 1036,
                                as_str(): "MyStruct",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1038,
                                    end: 1039,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 1041,
                                        end: 1047,
                                        as_str(): "MyEnum",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1041,
                                end: 1047,
                                as_str(): "MyEnum",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1049,
                                    end: 1050,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 1052,
                                        end: 1068,
                                        as_str(): "MyStructWithEnum",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1052,
                                end: 1068,
                                as_str(): "MyStructWithEnum",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1070,
                                    end: 1071,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Str(
                                Length {
                                    val: 5,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 1077,
                                        end: 1078,
                                        as_str(): "5",
                                    },
                                },
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1073,
                                end: 1079,
                                as_str(): "str[5]",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1081,
                                    end: 1082,
                                    as_str(): "e",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Boolean,
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1084,
                                end: 1088,
                                as_str(): "bool",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1090,
                                    end: 1091,
                                    as_str(): "f",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1093,
                                end: 1096,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1098,
                                    end: 1099,
                                    as_str(): "g",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: UnsignedInteger(
                                Eight,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1101,
                                end: 1103,
                                as_str(): "u8",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1105,
                                    end: 1106,
                                    as_str(): "h",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fb14c011bb0,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: B256,
                            type_span: Span {
                                src (ptr): 0x00007fb13d3811d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                ),
                                start: 1108,
                                end: 1112,
                                as_str(): "b256",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb13d3811d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                        ),
                        start: 1011,
                        end: 1129,
                        as_str(): "fn eight_args(a: MyStruct, b: MyEnum, c: MyStructWithEnum, d: str[5], e: bool, f: u64, g: u8, h: b256) {\n    return;\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb13d3811d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                        ),
                        start: 1011,
                        end: 1113,
                        as_str(): "fn eight_args(a: MyStruct, b: MyEnum, c: MyStructWithEnum, d: str[5], e: bool, f: u64, g: u8, h: b256)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 1011,
            end: 1129,
            as_str(): "fn eight_args(a: MyStruct, b: MyEnum, c: MyStructWithEnum, d: str[5], e: bool, f: u64, g: u8, h: b256) {\n    return;\n}",
        },
    },
]
