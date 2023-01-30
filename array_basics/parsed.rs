[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 16,
                            end: 17,
                            as_str(): "S",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 24,
                                    end: 27,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb14c154030,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                ),
                                start: 24,
                                end: 32,
                                as_str(): "foo: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb14c154030,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                ),
                                start: 29,
                                end: 32,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 38,
                                    end: 41,
                                    as_str(): "bar",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb14c154030,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                ),
                                start: 38,
                                end: 46,
                                as_str(): "bar: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb14c154030,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
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
                        src (ptr): 0x00007fb14c154030,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                        ),
                        start: 9,
                        end: 49,
                        as_str(): "struct S {\n    foo: u64,\n    bar: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14c154030,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
            ),
            start: 9,
            end: 49,
            as_str(): "struct S {\n    foo: u64,\n    bar: u64,\n}",
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
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 54,
                            end: 58,
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
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 79,
                                                    end: 80,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        71,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        71,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 83,
                                                        end: 87,
                                                        as_str(): "bool",
                                                    },
                                                },
                                                Length {
                                                    val: 5,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 93,
                                                        end: 94,
                                                        as_str(): "5",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 82,
                                                    end: 95,
                                                    as_str(): "[bool;\n    5]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 99,
                                                                    end: 103,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 109,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 111,
                                                                    end: 115,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        false,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 122,
                                                                    as_str(): "false",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        true,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 124,
                                                                    end: 128,
                                                                    as_str(): "true",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 98,
                                                    end: 129,
                                                    as_str(): "[true, true, true, false, true]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 75,
                                    end: 130,
                                    as_str(): "let a: [bool;\n    5] = [true, true, true, false, true];",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 139,
                                                    end: 140,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        33,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        33,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 143,
                                                        end: 146,
                                                        as_str(): "u32",
                                                    },
                                                },
                                                Length {
                                                    val: 10,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 152,
                                                        end: 154,
                                                        as_str(): "10",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 142,
                                                    end: 155,
                                                    as_str(): "[u32;\n    10]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 160,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        ],
                                                        length_span: Some(
                                                            Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 166,
                                                                end: 168,
                                                                as_str(): "10",
                                                            },
                                                        ),
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 158,
                                                    end: 169,
                                                    as_str(): "[3;\n    10]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 170,
                                    as_str(): "let b: [u32;\n    10] = [3;\n    10];",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 179,
                                                    end: 180,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 184,
                                                                    end: 188,
                                                                    as_str(): "0x01",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        2,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 190,
                                                                    end: 194,
                                                                    as_str(): "0x02",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 196,
                                                                    end: 200,
                                                                    as_str(): "0x03",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 183,
                                                    end: 201,
                                                    as_str(): "[0x01, 0x02, 0x03]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 175,
                                    end: 202,
                                    as_str(): "let c = [0x01, 0x02, 0x03];",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 211,
                                                    end: 212,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        0,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "0",
                                                                },
                                                            },
                                                        ],
                                                        length_span: Some(
                                                            Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 223,
                                                                end: 225,
                                                                as_str(): "10",
                                                            },
                                                        ),
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 215,
                                                    end: 226,
                                                    as_str(): "[0;\n    10]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 207,
                                    end: 227,
                                    as_str(): "let d = [0;\n    10];",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 236,
                                                    end: 237,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        7249,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        7249,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 240,
                                                        end: 252,
                                                        as_str(): "[u64;\n    4]",
                                                    },
                                                },
                                                Length {
                                                    val: 2,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 258,
                                                        end: 259,
                                                        as_str(): "2",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 239,
                                                    end: 260,
                                                    as_str(): "[[u64;\n    4];\n    2]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Array(
                                                                    ArrayExpression {
                                                                        contents: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        1,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 265,
                                                                                    end: 266,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        2,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 268,
                                                                                    end: 269,
                                                                                    as_str(): "2",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 271,
                                                                                    end: 272,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        4,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 275,
                                                                                    as_str(): "4",
                                                                                },
                                                                            },
                                                                        ],
                                                                        length_span: None,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 264,
                                                                    end: 276,
                                                                    as_str(): "[1, 2, 3, 4]",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Array(
                                                                    ArrayExpression {
                                                                        contents: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 279,
                                                                                    end: 280,
                                                                                    as_str(): "5",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        6,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 282,
                                                                                    end: 283,
                                                                                    as_str(): "6",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        7,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 285,
                                                                                    end: 286,
                                                                                    as_str(): "7",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        8,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 288,
                                                                                    end: 289,
                                                                                    as_str(): "8",
                                                                                },
                                                                            },
                                                                        ],
                                                                        length_span: None,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 278,
                                                                    end: 290,
                                                                    as_str(): "[5, 6, 7, 8]",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 263,
                                                    end: 291,
                                                    as_str(): "[[1, 2, 3, 4], [5, 6, 7, 8]]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 232,
                                    end: 292,
                                    as_str(): "let e: [[u64;\n    4];\n    2] = [[1, 2, 3, 4], [5, 6, 7, 8]];",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 337,
                                                    end: 338,
                                                    as_str(): "g",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Struct(
                                                                    StructExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 342,
                                                                                        end: 343,
                                                                                        as_str(): "S",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 342,
                                                                                end: 343,
                                                                                as_str(): "S",
                                                                            },
                                                                        },
                                                                        fields: [
                                                                            StructExpressionField {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 354,
                                                                                        end: 357,
                                                                                        as_str(): "foo",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            10,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 359,
                                                                                        end: 361,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 354,
                                                                                    end: 361,
                                                                                    as_str(): "foo: 10",
                                                                                },
                                                                            },
                                                                            StructExpressionField {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 363,
                                                                                        end: 366,
                                                                                        as_str(): "bar",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            20,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 368,
                                                                                        end: 370,
                                                                                        as_str(): "20",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 363,
                                                                                    end: 370,
                                                                                    as_str(): "bar: 20",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 342,
                                                                    end: 376,
                                                                    as_str(): "S {\n        foo: 10, bar: 20\n    }",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Struct(
                                                                    StructExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 382,
                                                                                        end: 383,
                                                                                        as_str(): "S",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 382,
                                                                                end: 383,
                                                                                as_str(): "S",
                                                                            },
                                                                        },
                                                                        fields: [
                                                                            StructExpressionField {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 394,
                                                                                        end: 397,
                                                                                        as_str(): "foo",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            1,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 399,
                                                                                        end: 400,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 400,
                                                                                    as_str(): "foo: 1",
                                                                                },
                                                                            },
                                                                            StructExpressionField {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 402,
                                                                                        end: 405,
                                                                                        as_str(): "bar",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            2,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 407,
                                                                                        end: 408,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 402,
                                                                                    end: 408,
                                                                                    as_str(): "bar: 2",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 382,
                                                                    end: 414,
                                                                    as_str(): "S {\n        foo: 1, bar: 2\n    }",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 341,
                                                    end: 420,
                                                    as_str(): "[S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 333,
                                    end: 421,
                                    as_str(): "let g = [S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ];",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 430,
                                                    end: 431,
                                                    as_str(): "h",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: ArrayIndex(
                                                    ArrayIndexExpression {
                                                        prefix: Expression {
                                                            kind: FunctionApplication(
                                                                FunctionApplicationExpression {
                                                                    call_path_binding: TypeBinding {
                                                                        inner: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 434,
                                                                                    end: 435,
                                                                                    as_str(): "i",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 434,
                                                                            end: 435,
                                                                            as_str(): "i",
                                                                        },
                                                                    },
                                                                    arguments: [],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 434,
                                                                end: 437,
                                                                as_str(): "i()",
                                                            },
                                                        },
                                                        index: Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    2,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 438,
                                                                end: 439,
                                                                as_str(): "2",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 434,
                                                    end: 440,
                                                    as_str(): "i()[2]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 426,
                                    end: 441,
                                    as_str(): "let h = i()[2];",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: LazyOperator(
                                            LazyOperatorExpression {
                                                op: And,
                                                lhs: Expression {
                                                    kind: LazyOperator(
                                                        LazyOperatorExpression {
                                                            op: And,
                                                            lhs: Expression {
                                                                kind: LazyOperator(
                                                                    LazyOperatorExpression {
                                                                        op: And,
                                                                        lhs: Expression {
                                                                            kind: LazyOperator(
                                                                                LazyOperatorExpression {
                                                                                    op: And,
                                                                                    lhs: Expression {
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
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 452,
                                                                                                                        end: 454,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 452,
                                                                                                                        end: 454,
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
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 452,
                                                                                                                    end: 454,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 452,
                                                                                                        end: 454,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: ArrayIndex(
                                                                                                            ArrayIndexExpression {
                                                                                                                prefix: Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 447,
                                                                                                                                end: 448,
                                                                                                                                as_str(): "b",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 447,
                                                                                                                        end: 448,
                                                                                                                        as_str(): "b",
                                                                                                                    },
                                                                                                                },
                                                                                                                index: Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            0,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 449,
                                                                                                                        end: 450,
                                                                                                                        as_str(): "0",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 447,
                                                                                                            end: 451,
                                                                                                            as_str(): "b[0]",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: ArrayIndex(
                                                                                                            ArrayIndexExpression {
                                                                                                                prefix: Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 455,
                                                                                                                                end: 456,
                                                                                                                                as_str(): "b",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 455,
                                                                                                                        end: 456,
                                                                                                                        as_str(): "b",
                                                                                                                    },
                                                                                                                },
                                                                                                                index: Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            9,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 457,
                                                                                                                        end: 458,
                                                                                                                        as_str(): "9",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 455,
                                                                                                            end: 459,
                                                                                                            as_str(): "b[9]",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 447,
                                                                                            end: 459,
                                                                                            as_str(): "b[0] == b[9]",
                                                                                        },
                                                                                    },
                                                                                    rhs: Expression {
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
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 481,
                                                                                                                        end: 483,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 481,
                                                                                                                        end: 483,
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
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 481,
                                                                                                                    end: 483,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 481,
                                                                                                        end: 483,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
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
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 471,
                                                                                                                                        end: 472,
                                                                                                                                        as_str(): "+",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: Some(
                                                                                                                                        "ops",
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 471,
                                                                                                                                        end: 472,
                                                                                                                                        as_str(): "+",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ],
                                                                                                                            suffix: BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "add",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 471,
                                                                                                                                    end: 472,
                                                                                                                                    as_str(): "+",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            is_absolute: true,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 471,
                                                                                                                        end: 472,
                                                                                                                        as_str(): "+",
                                                                                                                    },
                                                                                                                },
                                                                                                                contract_call_params: [],
                                                                                                                arguments: [
                                                                                                                    Expression {
                                                                                                                        kind: ArrayIndex(
                                                                                                                            ArrayIndexExpression {
                                                                                                                                prefix: Expression {
                                                                                                                                    kind: ArrayIndex(
                                                                                                                                        ArrayIndexExpression {
                                                                                                                                            prefix: Expression {
                                                                                                                                                kind: Variable(
                                                                                                                                                    BaseIdent {
                                                                                                                                                        name_override_opt: None,
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 463,
                                                                                                                                                            end: 464,
                                                                                                                                                            as_str(): "e",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 463,
                                                                                                                                                    end: 464,
                                                                                                                                                    as_str(): "e",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                            index: Expression {
                                                                                                                                                kind: Literal(
                                                                                                                                                    Numeric(
                                                                                                                                                        0,
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 465,
                                                                                                                                                    end: 466,
                                                                                                                                                    as_str(): "0",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 463,
                                                                                                                                        end: 467,
                                                                                                                                        as_str(): "e[0]",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                index: Expression {
                                                                                                                                    kind: Literal(
                                                                                                                                        Numeric(
                                                                                                                                            1,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 468,
                                                                                                                                        end: 469,
                                                                                                                                        as_str(): "1",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 463,
                                                                                                                            end: 470,
                                                                                                                            as_str(): "e[0][1]",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Expression {
                                                                                                                        kind: ArrayIndex(
                                                                                                                            ArrayIndexExpression {
                                                                                                                                prefix: Expression {
                                                                                                                                    kind: ArrayIndex(
                                                                                                                                        ArrayIndexExpression {
                                                                                                                                            prefix: Expression {
                                                                                                                                                kind: Variable(
                                                                                                                                                    BaseIdent {
                                                                                                                                                        name_override_opt: None,
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 473,
                                                                                                                                                            end: 474,
                                                                                                                                                            as_str(): "e",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 473,
                                                                                                                                                    end: 474,
                                                                                                                                                    as_str(): "e",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                            index: Expression {
                                                                                                                                                kind: Literal(
                                                                                                                                                    Numeric(
                                                                                                                                                        1,
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 475,
                                                                                                                                                    end: 476,
                                                                                                                                                    as_str(): "1",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 473,
                                                                                                                                        end: 477,
                                                                                                                                        as_str(): "e[1]",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                index: Expression {
                                                                                                                                    kind: Literal(
                                                                                                                                        Numeric(
                                                                                                                                            2,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 478,
                                                                                                                                        end: 479,
                                                                                                                                        as_str(): "2",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 473,
                                                                                                                            end: 480,
                                                                                                                            as_str(): "e[1][2]",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 463,
                                                                                                            end: 480,
                                                                                                            as_str(): "e[0][1] + e[1][2]",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                9,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 484,
                                                                                                            end: 485,
                                                                                                            as_str(): "9",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 463,
                                                                                            end: 485,
                                                                                            as_str(): "e[0][1] + e[1][2] == 9",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 447,
                                                                                end: 485,
                                                                                as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9",
                                                                            },
                                                                        },
                                                                        rhs: Expression {
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
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 509,
                                                                                                            end: 511,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ops",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 509,
                                                                                                            end: 511,
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
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 509,
                                                                                                        end: 511,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: true,
                                                                                            },
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 509,
                                                                                            end: 511,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                    },
                                                                                    contract_call_params: [],
                                                                                    arguments: [
                                                                                        Expression {
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
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 498,
                                                                                                                            end: 499,
                                                                                                                            as_str(): "+",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 498,
                                                                                                                            end: 499,
                                                                                                                            as_str(): "+",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ],
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "add",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 498,
                                                                                                                        end: 499,
                                                                                                                        as_str(): "+",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                is_absolute: true,
                                                                                                            },
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 498,
                                                                                                            end: 499,
                                                                                                            as_str(): "+",
                                                                                                        },
                                                                                                    },
                                                                                                    contract_call_params: [],
                                                                                                    arguments: [
                                                                                                        Expression {
                                                                                                            kind: Subfield(
                                                                                                                SubfieldExpression {
                                                                                                                    prefix: Expression {
                                                                                                                        kind: ArrayIndex(
                                                                                                                            ArrayIndexExpression {
                                                                                                                                prefix: Expression {
                                                                                                                                    kind: Variable(
                                                                                                                                        BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 489,
                                                                                                                                                end: 490,
                                                                                                                                                as_str(): "g",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 489,
                                                                                                                                        end: 490,
                                                                                                                                        as_str(): "g",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                index: Expression {
                                                                                                                                    kind: Literal(
                                                                                                                                        Numeric(
                                                                                                                                            0,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 491,
                                                                                                                                        end: 492,
                                                                                                                                        as_str(): "0",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 489,
                                                                                                                            end: 493,
                                                                                                                            as_str(): "g[0]",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    field_to_access: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 494,
                                                                                                                            end: 497,
                                                                                                                            as_str(): "foo",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 489,
                                                                                                                end: 497,
                                                                                                                as_str(): "g[0].foo",
                                                                                                            },
                                                                                                        },
                                                                                                        Expression {
                                                                                                            kind: Subfield(
                                                                                                                SubfieldExpression {
                                                                                                                    prefix: Expression {
                                                                                                                        kind: ArrayIndex(
                                                                                                                            ArrayIndexExpression {
                                                                                                                                prefix: Expression {
                                                                                                                                    kind: Variable(
                                                                                                                                        BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 500,
                                                                                                                                                end: 501,
                                                                                                                                                as_str(): "g",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 500,
                                                                                                                                        end: 501,
                                                                                                                                        as_str(): "g",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                index: Expression {
                                                                                                                                    kind: Literal(
                                                                                                                                        Numeric(
                                                                                                                                            1,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 502,
                                                                                                                                        end: 503,
                                                                                                                                        as_str(): "1",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 500,
                                                                                                                            end: 504,
                                                                                                                            as_str(): "g[1]",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    field_to_access: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 505,
                                                                                                                            end: 508,
                                                                                                                            as_str(): "bar",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 500,
                                                                                                                end: 508,
                                                                                                                as_str(): "g[1].bar",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 489,
                                                                                                end: 508,
                                                                                                as_str(): "g[0].foo + g[1].bar",
                                                                                            },
                                                                                        },
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    12,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 512,
                                                                                                end: 514,
                                                                                                as_str(): "12",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 489,
                                                                                end: 514,
                                                                                as_str(): "g[0].foo + g[1].bar == 12",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 447,
                                                                    end: 514,
                                                                    as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12",
                                                                },
                                                            },
                                                            rhs: Expression {
                                                                kind: FunctionApplication(
                                                                    FunctionApplicationExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 518,
                                                                                        end: 519,
                                                                                        as_str(): "j",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 518,
                                                                                end: 519,
                                                                                as_str(): "j",
                                                                            },
                                                                        },
                                                                        arguments: [
                                                                            Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 520,
                                                                                            end: 521,
                                                                                            as_str(): "g",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 520,
                                                                                    end: 521,
                                                                                    as_str(): "g",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 518,
                                                                    end: 522,
                                                                    as_str(): "j(g)",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 447,
                                                        end: 522,
                                                        as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g)",
                                                    },
                                                },
                                                rhs: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 552,
                                                        end: 556,
                                                        as_str(): "true",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 447,
                                            end: 556,
                                            as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 447,
                                    end: 556,
                                    as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 69,
                            end: 558,
                            as_str(): "{\n    let a: [bool;\n    5] = [true, true, true, false, true];\n    let b: [u32;\n    10] = [3;\n    10];\n    let c = [0x01, 0x02, 0x03];\n    let d = [0;\n    10];\n    let e: [[u64;\n    4];\n    2] = [[1, 2, 3, 4], [5, 6, 7, 8]];\n    //let f: [u64; 1 + 1] = [0, 0];\n    let g = [S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ];\n    let h = i()[2];\n\n    b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb14c154030,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                        ),
                        start: 51,
                        end: 558,
                        as_str(): "fn main() -> bool {\n    let a: [bool;\n    5] = [true, true, true, false, true];\n    let b: [u32;\n    10] = [3;\n    10];\n    let c = [0x01, 0x02, 0x03];\n    let d = [0;\n    10];\n    let e: [[u64;\n    4];\n    2] = [[1, 2, 3, 4], [5, 6, 7, 8]];\n    //let f: [u64; 1 + 1] = [0, 0];\n    let g = [S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ];\n    let h = i()[2];\n\n    b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb14c154030,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                        ),
                        start: 64,
                        end: 68,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14c154030,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
            ),
            start: 51,
            end: 558,
            as_str(): "fn main() -> bool {\n    let a: [bool;\n    5] = [true, true, true, false, true];\n    let b: [u32;\n    10] = [3;\n    10];\n    let c = [0x01, 0x02, 0x03];\n    let d = [0;\n    10];\n    let e: [[u64;\n    4];\n    2] = [[1, 2, 3, 4], [5, 6, 7, 8]];\n    //let f: [u64; 1 + 1] = [0, 0];\n    let g = [S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ];\n    let h = i()[2];\n\n    b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true\n}",
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
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 563,
                            end: 564,
                            as_str(): "i",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Array(
                                            ArrayExpression {
                                                contents: [
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                0,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 586,
                                                            end: 587,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                1,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 589,
                                                            end: 590,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                2,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 592,
                                                            end: 593,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                3,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 595,
                                                            end: 596,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ],
                                                length_span: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 585,
                                            end: 597,
                                            as_str(): "[0, 1, 2, 3]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 585,
                                    end: 597,
                                    as_str(): "[0, 1, 2, 3]",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 579,
                            end: 599,
                            as_str(): "{\n    [0, 1, 2, 3]\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb14c154030,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                        ),
                        start: 560,
                        end: 599,
                        as_str(): "fn i() -> [u64;\n4] {\n    [0, 1, 2, 3]\n}",
                    },
                    return_type: Array(
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb14c154030,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                ),
                                start: 571,
                                end: 574,
                                as_str(): "u64",
                            },
                        },
                        Length {
                            val: 4,
                            span: Span {
                                src (ptr): 0x00007fb14c154030,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                ),
                                start: 576,
                                end: 577,
                                as_str(): "4",
                            },
                        },
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb14c154030,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                        ),
                        start: 570,
                        end: 578,
                        as_str(): "[u64;\n4]",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14c154030,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
            ),
            start: 560,
            end: 599,
            as_str(): "fn i() -> [u64;\n4] {\n    [0, 1, 2, 3]\n}",
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
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 604,
                            end: 605,
                            as_str(): "j",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
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
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 669,
                                                                        end: 671,
                                                                        as_str(): "==",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "ops",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 669,
                                                                        end: 671,
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
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 669,
                                                                    end: 671,
                                                                    as_str(): "==",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_absolute: true,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 669,
                                                        end: 671,
                                                        as_str(): "==",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
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
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 652,
                                                                                        end: 653,
                                                                                        as_str(): "+",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 652,
                                                                                        end: 653,
                                                                                        as_str(): "+",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "add",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 652,
                                                                                    end: 653,
                                                                                    as_str(): "+",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 652,
                                                                        end: 653,
                                                                        as_str(): "+",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: ArrayIndex(
                                                                                        ArrayIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 637,
                                                                                                            end: 644,
                                                                                                            as_str(): "ary_arg",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 637,
                                                                                                    end: 644,
                                                                                                    as_str(): "ary_arg",
                                                                                                },
                                                                                            },
                                                                                            index: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 645,
                                                                                                    end: 646,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 637,
                                                                                        end: 647,
                                                                                        as_str(): "ary_arg[0]",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 648,
                                                                                        end: 651,
                                                                                        as_str(): "foo",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 637,
                                                                            end: 651,
                                                                            as_str(): "ary_arg[0].foo",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: ArrayIndex(
                                                                                        ArrayIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 654,
                                                                                                            end: 661,
                                                                                                            as_str(): "ary_arg",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 654,
                                                                                                    end: 661,
                                                                                                    as_str(): "ary_arg",
                                                                                                },
                                                                                            },
                                                                                            index: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 662,
                                                                                                    end: 663,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 654,
                                                                                        end: 664,
                                                                                        as_str(): "ary_arg[1]",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 665,
                                                                                        end: 668,
                                                                                        as_str(): "bar",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 654,
                                                                            end: 668,
                                                                            as_str(): "ary_arg[1].bar",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 637,
                                                            end: 668,
                                                            as_str(): "ary_arg[0].foo + ary_arg[1].bar",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                12,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 672,
                                                            end: 674,
                                                            as_str(): "12",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 637,
                                            end: 674,
                                            as_str(): "ary_arg[0].foo + ary_arg[1].bar == 12",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 637,
                                    end: 674,
                                    as_str(): "ary_arg[0].foo + ary_arg[1].bar == 12",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 631,
                            end: 676,
                            as_str(): "{\n    ary_arg[0].foo + ary_arg[1].bar == 12\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 606,
                                    end: 613,
                                    as_str(): "ary_arg",
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
                            type_info: Array(
                                TypeArgument {
                                    type_id: TypeId(
                                        7250,
                                    ),
                                    initial_type_id: TypeId(
                                        7250,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 616,
                                        end: 617,
                                        as_str(): "S",
                                    },
                                },
                                Length {
                                    val: 2,
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 619,
                                        end: 620,
                                        as_str(): "2",
                                    },
                                },
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb14c154030,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                ),
                                start: 615,
                                end: 621,
                                as_str(): "[S;\n2]",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb14c154030,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                        ),
                        start: 601,
                        end: 676,
                        as_str(): "fn j(ary_arg: [S;\n2]) -> bool {\n    ary_arg[0].foo + ary_arg[1].bar == 12\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb14c154030,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                        ),
                        start: 626,
                        end: 630,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14c154030,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
            ),
            start: 601,
            end: 676,
            as_str(): "fn j(ary_arg: [S;\n2]) -> bool {\n    ary_arg[0].foo + ary_arg[1].bar == 12\n}",
        },
    },
]
