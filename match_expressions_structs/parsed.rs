[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe092b73f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                            ),
                            start: 16,
                            end: 21,
                            as_str(): "Point",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 28,
                                    end: 29,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe092b73f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                ),
                                start: 28,
                                end: 34,
                                as_str(): "x: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe092b73f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                ),
                                start: 31,
                                end: 34,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 40,
                                    end: 41,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe092b73f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                ),
                                start: 40,
                                end: 46,
                                as_str(): "y: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe092b73f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                ),
                                start: 43,
                                end: 46,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe092b73f20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                        ),
                        start: 9,
                        end: 48,
                        as_str(): "struct Point {\n    x: u64,\n    y: u64\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe092b73f20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
            ),
            start: 9,
            end: 48,
            as_str(): "struct Point {\n    x: u64,\n    y: u64\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe092b73f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                            ),
                            start: 57,
                            end: 61,
                            as_str(): "Data",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 71,
                                    end: 76,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 78,
                                        end: 79,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe092b73f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                ),
                                start: 71,
                                end: 79,
                                as_str(): "value: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe092b73f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                ),
                                start: 78,
                                end: 79,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7249),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe092b73f20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                        ),
                        start: 50,
                        end: 81,
                        as_str(): "struct Data<T> {\n    value: T\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe092b73f20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
            ),
            start: 50,
            end: 81,
            as_str(): "struct Data<T> {\n    value: T\n}",
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
                            src (ptr): 0x00007fe092b73f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                            ),
                            start: 86,
                            end: 90,
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
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 110,
                                                    end: 111,
                                                    as_str(): "a",
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
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 114,
                                                                        end: 119,
                                                                        as_str(): "Point",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 114,
                                                                end: 119,
                                                                as_str(): "Point",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 130,
                                                                        end: 131,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 133,
                                                                        end: 134,
                                                                        as_str(): "3",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 130,
                                                                    end: 134,
                                                                    as_str(): "x: 3",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 144,
                                                                        end: 145,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 147,
                                                                        end: 148,
                                                                        as_str(): "4",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 144,
                                                                    end: 148,
                                                                    as_str(): "y: 4",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 114,
                                                    end: 155,
                                                    as_str(): "Point {\n        x: 3,\n        y: 4,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 106,
                                    end: 156,
                                    as_str(): "let a = Point {\n        x: 3,\n        y: 4,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 165,
                                                    end: 166,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: CodeBlock(
                                                    CodeBlock {
                                                        contents: [
                                                            AstNode {
                                                                content: Declaration(
                                                                    VariableDeclaration(
                                                                        VariableDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_1",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 175,
                                                                                    end: 176,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_ascription: Unknown,
                                                                            type_ascription_span: None,
                                                                            body: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 175,
                                                                                            end: 176,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 175,
                                                                                    end: 176,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 169,
                                                                    end: 281,
                                                                    as_str(): "match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                                                },
                                                            },
                                                            AstNode {
                                                                content: ImplicitReturnExpression(
                                                                    Expression {
                                                                        kind: Match(
                                                                            MatchExpression {
                                                                                value: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_1",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 175,
                                                                                                end: 176,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 175,
                                                                                        end: 176,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: StructScrutinee {
                                                                                            struct_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 187,
                                                                                                    end: 192,
                                                                                                    as_str(): "Point",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            fields: [
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 195,
                                                                                                            end: 196,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: Some(
                                                                                                        Literal {
                                                                                                            value: Numeric(
                                                                                                                3,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 198,
                                                                                                                end: 199,
                                                                                                                as_str(): "3",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 195,
                                                                                                        end: 199,
                                                                                                        as_str(): "x: 3",
                                                                                                    },
                                                                                                },
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 201,
                                                                                                            end: 202,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 201,
                                                                                                        end: 202,
                                                                                                        as_str(): "y",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 187,
                                                                                                end: 204,
                                                                                                as_str(): "Point { x: 3, y }",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 210,
                                                                                                                                end: 211,
                                                                                                                                as_str(): "y",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 210,
                                                                                                                        end: 211,
                                                                                                                        as_str(): "y",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 210,
                                                                                                                end: 211,
                                                                                                                as_str(): "y",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 208,
                                                                                                        end: 213,
                                                                                                        as_str(): "{ y }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 208,
                                                                                                end: 213,
                                                                                                as_str(): "{ y }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 187,
                                                                                            end: 214,
                                                                                            as_str(): "Point { x: 3, y } => { y },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: StructScrutinee {
                                                                                            struct_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 223,
                                                                                                    end: 228,
                                                                                                    as_str(): "Point",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            fields: [
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 231,
                                                                                                            end: 232,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: Some(
                                                                                                        Literal {
                                                                                                            value: Numeric(
                                                                                                                3,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 234,
                                                                                                                end: 235,
                                                                                                                as_str(): "3",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 231,
                                                                                                        end: 235,
                                                                                                        as_str(): "x: 3",
                                                                                                    },
                                                                                                },
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 237,
                                                                                                            end: 238,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: Some(
                                                                                                        Literal {
                                                                                                            value: Numeric(
                                                                                                                4,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 240,
                                                                                                                end: 241,
                                                                                                                as_str(): "4",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 237,
                                                                                                        end: 241,
                                                                                                        as_str(): "y: 4",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 223,
                                                                                                end: 243,
                                                                                                as_str(): "Point { x: 3, y: 4 }",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            24,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 249,
                                                                                                                        end: 251,
                                                                                                                        as_str(): "24",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 249,
                                                                                                                end: 251,
                                                                                                                as_str(): "24",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 247,
                                                                                                        end: 253,
                                                                                                        as_str(): "{ 24 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 247,
                                                                                                end: 253,
                                                                                                as_str(): "{ 24 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 223,
                                                                                            end: 254,
                                                                                            as_str(): "Point { x: 3, y: 4 } => { 24 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 263,
                                                                                                end: 264,
                                                                                                as_str(): "_",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            24,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 270,
                                                                                                                        end: 272,
                                                                                                                        as_str(): "24",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 270,
                                                                                                                end: 272,
                                                                                                                as_str(): "24",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 268,
                                                                                                        end: 274,
                                                                                                        as_str(): "{ 24 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 268,
                                                                                                end: 274,
                                                                                                as_str(): "{ 24 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 263,
                                                                                            end: 275,
                                                                                            as_str(): "_ => { 24 },",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 169,
                                                                            end: 281,
                                                                            as_str(): "match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 169,
                                                                    end: 281,
                                                                    as_str(): "match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 281,
                                                            as_str(): "match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 281,
                                                    as_str(): "match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 161,
                                    end: 282,
                                    as_str(): "let b = match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 292,
                                                    end: 293,
                                                    as_str(): "c",
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
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 296,
                                                                        end: 300,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe092b73f20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                ),
                                                                start: 296,
                                                                end: 300,
                                                                as_str(): "Data",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 311,
                                                                        end: 316,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Boolean(
                                                                            true,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 318,
                                                                        end: 322,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 311,
                                                                    end: 322,
                                                                    as_str(): "value: true",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 296,
                                                    end: 328,
                                                    as_str(): "Data {\n        value: true\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 288,
                                    end: 329,
                                    as_str(): "let c = Data {\n        value: true\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 338,
                                                    end: 339,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: CodeBlock(
                                                    CodeBlock {
                                                        contents: [
                                                            AstNode {
                                                                content: Declaration(
                                                                    VariableDeclaration(
                                                                        VariableDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_2",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 348,
                                                                                    end: 349,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_ascription: Unknown,
                                                                            type_ascription_span: None,
                                                                            body: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 348,
                                                                                            end: 349,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 348,
                                                                                    end: 349,
                                                                                    as_str(): "c",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 342,
                                                                    end: 430,
                                                                    as_str(): "match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                                                },
                                                            },
                                                            AstNode {
                                                                content: ImplicitReturnExpression(
                                                                    Expression {
                                                                        kind: Match(
                                                                            MatchExpression {
                                                                                value: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_2",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 348,
                                                                                                end: 349,
                                                                                                as_str(): "c",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 348,
                                                                                        end: 349,
                                                                                        as_str(): "c",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: StructScrutinee {
                                                                                            struct_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 360,
                                                                                                    end: 364,
                                                                                                    as_str(): "Data",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            fields: [
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 367,
                                                                                                            end: 372,
                                                                                                            as_str(): "value",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: Some(
                                                                                                        Literal {
                                                                                                            value: Boolean(
                                                                                                                false,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 374,
                                                                                                                end: 379,
                                                                                                                as_str(): "false",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 367,
                                                                                                        end: 379,
                                                                                                        as_str(): "value: false",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 360,
                                                                                                end: 381,
                                                                                                as_str(): "Data { value: false }",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            0,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 387,
                                                                                                                        end: 388,
                                                                                                                        as_str(): "0",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 387,
                                                                                                                end: 388,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 385,
                                                                                                        end: 390,
                                                                                                        as_str(): "{ 0 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 385,
                                                                                                end: 390,
                                                                                                as_str(): "{ 0 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 360,
                                                                                            end: 391,
                                                                                            as_str(): "Data { value: false } => { 0 },",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: StructScrutinee {
                                                                                            struct_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 400,
                                                                                                    end: 404,
                                                                                                    as_str(): "Data",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            fields: [
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 407,
                                                                                                            end: 412,
                                                                                                            as_str(): "value",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 407,
                                                                                                        end: 412,
                                                                                                        as_str(): "value",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 400,
                                                                                                end: 414,
                                                                                                as_str(): "Data { value }",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: CodeBlock(
                                                                                                CodeBlock {
                                                                                                    contents: [
                                                                                                        AstNode {
                                                                                                            content: ImplicitReturnExpression(
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            4,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 420,
                                                                                                                        end: 421,
                                                                                                                        as_str(): "4",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 420,
                                                                                                                end: 421,
                                                                                                                as_str(): "4",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                    whole_block_span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 418,
                                                                                                        end: 423,
                                                                                                        as_str(): "{ 4 }",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 418,
                                                                                                end: 423,
                                                                                                as_str(): "{ 4 }",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 400,
                                                                                            end: 424,
                                                                                            as_str(): "Data { value } => { 4 },",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 342,
                                                                            end: 430,
                                                                            as_str(): "match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 342,
                                                                    end: 430,
                                                                    as_str(): "match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 342,
                                                            end: 430,
                                                            as_str(): "match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 342,
                                                    end: 430,
                                                    as_str(): "match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 334,
                                    end: 431,
                                    as_str(): "let d = match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    };",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 437,
                                                    end: 438,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe092b73f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                            ),
                                            start: 437,
                                            end: 438,
                                            as_str(): "d",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 437,
                                    end: 438,
                                    as_str(): "d",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe092b73f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                            ),
                            start: 100,
                            end: 440,
                            as_str(): "{\n    let a = Point {\n        x: 3,\n        y: 4,\n    };\n    let b = match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    };\n\n    let c = Data {\n        value: true\n    };\n    let d = match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    };\n\n    d\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe092b73f20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                        ),
                        start: 83,
                        end: 440,
                        as_str(): "fn main() -> u64 {\n    let a = Point {\n        x: 3,\n        y: 4,\n    };\n    let b = match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    };\n\n    let c = Data {\n        value: true\n    };\n    let d = match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    };\n\n    d\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe092b73f20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                        ),
                        start: 96,
                        end: 99,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe092b73f20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
            ),
            start: 83,
            end: 440,
            as_str(): "fn main() -> u64 {\n    let a = Point {\n        x: 3,\n        y: 4,\n    };\n    let b = match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    };\n\n    let c = Data {\n        value: true\n    };\n    let d = match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    };\n\n    d\n}",
        },
    },
]
