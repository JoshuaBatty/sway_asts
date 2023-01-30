[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 18,
                            end: 22,
                            as_str(): "hash",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 25,
                            end: 34,
                            as_str(): "keccak256",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb14cbeadf0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
            ),
            start: 9,
            end: 44,
            as_str(): "use std::hash::{keccak256, sha256};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 18,
                            end: 22,
                            as_str(): "hash",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 36,
                            end: 42,
                            as_str(): "sha256",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb14cbeadf0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
            ),
            start: 9,
            end: 44,
            as_str(): "use std::hash::{keccak256, sha256};",
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
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 49,
                            end: 53,
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
                                                    src (ptr): 0x00007fb14cbeadf0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                    ),
                                                    start: 73,
                                                    end: 77,
                                                    as_str(): "aaaa",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
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
                                                    src (ptr): 0x00007fb14cbeadf0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                    ),
                                                    start: 80,
                                                    end: 146,
                                                    as_str(): "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14cbeadf0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                    ),
                                    start: 69,
                                    end: 147,
                                    as_str(): "let aaaa = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14cbeadf0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                    ),
                                                    start: 156,
                                                    end: 160,
                                                    as_str(): "aaab",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
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
                                                            171,
                                                        ],
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14cbeadf0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                    ),
                                                    start: 163,
                                                    end: 230,
                                                    as_str(): "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_b",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14cbeadf0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                    ),
                                    start: 152,
                                    end: 231,
                                    as_str(): "let aaab = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_b;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb14cbeadf0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                    ),
                                                    start: 240,
                                                    end: 244,
                                                    as_str(): "abaa",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    B256(
                                                        [
                                                            171,
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
                                                    src (ptr): 0x00007fb14cbeadf0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                    ),
                                                    start: 247,
                                                    end: 315,
                                                    as_str(): "0xa_b_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14cbeadf0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                    ),
                                    start: 236,
                                    end: 316,
                                    as_str(): "let abaa = 0xa_b_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
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
                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                    ),
                                                                                    start: 329,
                                                                                    end: 331,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                    ),
                                                                                    start: 329,
                                                                                    end: 331,
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
                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                ),
                                                                                start: 329,
                                                                                end: 331,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                    ),
                                                                    start: 329,
                                                                    end: 331,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                ),
                                                                                start: 324,
                                                                                end: 328,
                                                                                as_str(): "aaaa",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                        ),
                                                                        start: 324,
                                                                        end: 328,
                                                                        as_str(): "aaaa",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                ),
                                                                                start: 332,
                                                                                end: 336,
                                                                                as_str(): "aaab",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                        ),
                                                                        start: 332,
                                                                        end: 336,
                                                                        as_str(): "aaab",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14cbeadf0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                        ),
                                                        start: 324,
                                                        end: 336,
                                                        as_str(): "aaaa == aaab",
                                                    },
                                                },
                                                then: Expression {
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
                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                ),
                                                                                start: 347,
                                                                                end: 348,
                                                                                as_str(): "0",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                        ),
                                                                        start: 347,
                                                                        end: 348,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                ),
                                                                start: 337,
                                                                end: 354,
                                                                as_str(): "{\n        0\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14cbeadf0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                        ),
                                                        start: 337,
                                                        end: 354,
                                                        as_str(): "{\n        0\n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: If(
                                                            IfExpression {
                                                                condition: Expression {
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
                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 368,
                                                                                                    end: 370,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 368,
                                                                                                    end: 370,
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
                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                ),
                                                                                                start: 368,
                                                                                                end: 370,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                    ),
                                                                                    start: 368,
                                                                                    end: 370,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                ),
                                                                                                start: 363,
                                                                                                end: 367,
                                                                                                as_str(): "aaaa",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                        ),
                                                                                        start: 363,
                                                                                        end: 367,
                                                                                        as_str(): "aaaa",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                ),
                                                                                                start: 371,
                                                                                                end: 375,
                                                                                                as_str(): "abaa",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                        ),
                                                                                        start: 371,
                                                                                        end: 375,
                                                                                        as_str(): "abaa",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                        ),
                                                                        start: 363,
                                                                        end: 375,
                                                                        as_str(): "aaaa == abaa",
                                                                    },
                                                                },
                                                                then: Expression {
                                                                    kind: CodeBlock(
                                                                        CodeBlock {
                                                                            contents: [
                                                                                AstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    1,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                ),
                                                                                                start: 386,
                                                                                                end: 387,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                        ),
                                                                                        start: 386,
                                                                                        end: 387,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            whole_block_span: Span {
                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                ),
                                                                                start: 376,
                                                                                end: 393,
                                                                                as_str(): "{\n        1\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                        ),
                                                                        start: 376,
                                                                        end: 393,
                                                                        as_str(): "{\n        1\n    }",
                                                                    },
                                                                },
                                                                else: Some(
                                                                    Expression {
                                                                        kind: If(
                                                                            IfExpression {
                                                                                condition: Expression {
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
                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 402,
                                                                                                                    end: 403,
                                                                                                                    as_str(): "!",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 402,
                                                                                                                    end: 403,
                                                                                                                    as_str(): "!",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "not",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 402,
                                                                                                                end: 403,
                                                                                                                as_str(): "!",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: true,
                                                                                                    },
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 402,
                                                                                                    end: 403,
                                                                                                    as_str(): "!",
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
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 409,
                                                                                                                                    end: 411,
                                                                                                                                    as_str(): "==",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "ops",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 409,
                                                                                                                                    end: 411,
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
                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 409,
                                                                                                                                end: 411,
                                                                                                                                as_str(): "==",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        is_absolute: true,
                                                                                                                    },
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 409,
                                                                                                                    end: 411,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                            },
                                                                                                            contract_call_params: [],
                                                                                                            arguments: [
                                                                                                                Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 404,
                                                                                                                                end: 408,
                                                                                                                                as_str(): "aaaa",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 404,
                                                                                                                        end: 408,
                                                                                                                        as_str(): "aaaa",
                                                                                                                    },
                                                                                                                },
                                                                                                                Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 412,
                                                                                                                                end: 416,
                                                                                                                                as_str(): "aaaa",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 412,
                                                                                                                        end: 416,
                                                                                                                        as_str(): "aaaa",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 404,
                                                                                                        end: 416,
                                                                                                        as_str(): "aaaa == aaaa",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                        ),
                                                                                        start: 402,
                                                                                        end: 417,
                                                                                        as_str(): "!(aaaa == aaaa)",
                                                                                    },
                                                                                },
                                                                                then: Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [
                                                                                                AstNode {
                                                                                                    content: ImplicitReturnExpression(
                                                                                                        Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    2,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 428,
                                                                                                                end: 429,
                                                                                                                as_str(): "2",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 428,
                                                                                                        end: 429,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                ),
                                                                                                start: 418,
                                                                                                end: 435,
                                                                                                as_str(): "{\n        2\n    }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                        ),
                                                                                        start: 418,
                                                                                        end: 435,
                                                                                        as_str(): "{\n        2\n    }",
                                                                                    },
                                                                                },
                                                                                else: Some(
                                                                                    Expression {
                                                                                        kind: If(
                                                                                            IfExpression {
                                                                                                condition: Expression {
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
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 444,
                                                                                                                                    end: 445,
                                                                                                                                    as_str(): "!",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "ops",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 444,
                                                                                                                                    end: 445,
                                                                                                                                    as_str(): "!",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        ],
                                                                                                                        suffix: BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "not",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 444,
                                                                                                                                end: 445,
                                                                                                                                as_str(): "!",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        is_absolute: true,
                                                                                                                    },
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 444,
                                                                                                                    end: 445,
                                                                                                                    as_str(): "!",
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
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 459,
                                                                                                                                                    end: 461,
                                                                                                                                                    as_str(): "==",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: Some(
                                                                                                                                                    "ops",
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 459,
                                                                                                                                                    end: 461,
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
                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 459,
                                                                                                                                                end: 461,
                                                                                                                                                as_str(): "==",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        is_absolute: true,
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                type_arguments: [],
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 459,
                                                                                                                                    end: 461,
                                                                                                                                    as_str(): "==",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            contract_call_params: [],
                                                                                                                            arguments: [
                                                                                                                                Expression {
                                                                                                                                    kind: FunctionApplication(
                                                                                                                                        FunctionApplicationExpression {
                                                                                                                                            call_path_binding: TypeBinding {
                                                                                                                                                inner: CallPath {
                                                                                                                                                    prefixes: [],
                                                                                                                                                    suffix: BaseIdent {
                                                                                                                                                        name_override_opt: None,
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 446,
                                                                                                                                                            end: 452,
                                                                                                                                                            as_str(): "sha256",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    is_absolute: false,
                                                                                                                                                },
                                                                                                                                                type_arguments: [],
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 446,
                                                                                                                                                    end: 452,
                                                                                                                                                    as_str(): "sha256",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                            arguments: [
                                                                                                                                                Expression {
                                                                                                                                                    kind: Variable(
                                                                                                                                                        BaseIdent {
                                                                                                                                                            name_override_opt: None,
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 453,
                                                                                                                                                                end: 457,
                                                                                                                                                                as_str(): "aaaa",
                                                                                                                                                            },
                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 453,
                                                                                                                                                        end: 457,
                                                                                                                                                        as_str(): "aaaa",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 446,
                                                                                                                                        end: 458,
                                                                                                                                        as_str(): "sha256(aaaa)",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                Expression {
                                                                                                                                    kind: Literal(
                                                                                                                                        B256(
                                                                                                                                            [
                                                                                                                                                224,
                                                                                                                                                231,
                                                                                                                                                122,
                                                                                                                                                80,
                                                                                                                                                116,
                                                                                                                                                18,
                                                                                                                                                177,
                                                                                                                                                32,
                                                                                                                                                246,
                                                                                                                                                237,
                                                                                                                                                230,
                                                                                                                                                31,
                                                                                                                                                98,
                                                                                                                                                41,
                                                                                                                                                91,
                                                                                                                                                26,
                                                                                                                                                123,
                                                                                                                                                47,
                                                                                                                                                241,
                                                                                                                                                157,
                                                                                                                                                61,
                                                                                                                                                204,
                                                                                                                                                143,
                                                                                                                                                114,
                                                                                                                                                83,
                                                                                                                                                229,
                                                                                                                                                22,
                                                                                                                                                99,
                                                                                                                                                71,
                                                                                                                                                12,
                                                                                                                                                136,
                                                                                                                                                142,
                                                                                                                                            ],
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 462,
                                                                                                                                        end: 528,
                                                                                                                                        as_str(): "0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ],
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 446,
                                                                                                                        end: 528,
                                                                                                                        as_str(): "sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 444,
                                                                                                        end: 529,
                                                                                                        as_str(): "!(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e)",
                                                                                                    },
                                                                                                },
                                                                                                then: Expression {
                                                                                                    kind: CodeBlock(
                                                                                                        CodeBlock {
                                                                                                            contents: [
                                                                                                                AstNode {
                                                                                                                    content: ImplicitReturnExpression(
                                                                                                                        Expression {
                                                                                                                            kind: Literal(
                                                                                                                                Numeric(
                                                                                                                                    3,
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 540,
                                                                                                                                end: 541,
                                                                                                                                as_str(): "3",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 540,
                                                                                                                        end: 541,
                                                                                                                        as_str(): "3",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                            whole_block_span: Span {
                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 530,
                                                                                                                end: 547,
                                                                                                                as_str(): "{\n        3\n    }",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 530,
                                                                                                        end: 547,
                                                                                                        as_str(): "{\n        3\n    }",
                                                                                                    },
                                                                                                },
                                                                                                else: Some(
                                                                                                    Expression {
                                                                                                        kind: If(
                                                                                                            IfExpression {
                                                                                                                condition: Expression {
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
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 556,
                                                                                                                                                    end: 557,
                                                                                                                                                    as_str(): "!",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: Some(
                                                                                                                                                    "ops",
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 556,
                                                                                                                                                    end: 557,
                                                                                                                                                    as_str(): "!",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ],
                                                                                                                                        suffix: BaseIdent {
                                                                                                                                            name_override_opt: Some(
                                                                                                                                                "not",
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 556,
                                                                                                                                                end: 557,
                                                                                                                                                as_str(): "!",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        is_absolute: true,
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                type_arguments: [],
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 556,
                                                                                                                                    end: 557,
                                                                                                                                    as_str(): "!",
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
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 574,
                                                                                                                                                                    end: 576,
                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                    "ops",
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 574,
                                                                                                                                                                    end: 576,
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
                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 574,
                                                                                                                                                                end: 576,
                                                                                                                                                                as_str(): "==",
                                                                                                                                                            },
                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                        },
                                                                                                                                                        is_absolute: true,
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                type_arguments: [],
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 574,
                                                                                                                                                    end: 576,
                                                                                                                                                    as_str(): "==",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                            contract_call_params: [],
                                                                                                                                            arguments: [
                                                                                                                                                Expression {
                                                                                                                                                    kind: FunctionApplication(
                                                                                                                                                        FunctionApplicationExpression {
                                                                                                                                                            call_path_binding: TypeBinding {
                                                                                                                                                                inner: CallPath {
                                                                                                                                                                    prefixes: [],
                                                                                                                                                                    suffix: BaseIdent {
                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 558,
                                                                                                                                                                            end: 567,
                                                                                                                                                                            as_str(): "keccak256",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                    is_absolute: false,
                                                                                                                                                                },
                                                                                                                                                                type_arguments: [],
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 558,
                                                                                                                                                                    end: 567,
                                                                                                                                                                    as_str(): "keccak256",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            arguments: [
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: Variable(
                                                                                                                                                                        BaseIdent {
                                                                                                                                                                            name_override_opt: None,
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 568,
                                                                                                                                                                                end: 572,
                                                                                                                                                                                as_str(): "aaaa",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 568,
                                                                                                                                                                        end: 572,
                                                                                                                                                                        as_str(): "aaaa",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 558,
                                                                                                                                                        end: 573,
                                                                                                                                                        as_str(): "keccak256(aaaa)",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Expression {
                                                                                                                                                    kind: Literal(
                                                                                                                                                        B256(
                                                                                                                                                            [
                                                                                                                                                                32,
                                                                                                                                                                238,
                                                                                                                                                                143,
                                                                                                                                                                19,
                                                                                                                                                                102,
                                                                                                                                                                240,
                                                                                                                                                                105,
                                                                                                                                                                38,
                                                                                                                                                                233,
                                                                                                                                                                232,
                                                                                                                                                                119,
                                                                                                                                                                29,
                                                                                                                                                                143,
                                                                                                                                                                185,
                                                                                                                                                                0,
                                                                                                                                                                122,
                                                                                                                                                                133,
                                                                                                                                                                55,
                                                                                                                                                                200,
                                                                                                                                                                223,
                                                                                                                                                                219,
                                                                                                                                                                106,
                                                                                                                                                                63,
                                                                                                                                                                140,
                                                                                                                                                                44,
                                                                                                                                                                253,
                                                                                                                                                                100,
                                                                                                                                                                219,
                                                                                                                                                                25,
                                                                                                                                                                210,
                                                                                                                                                                236,
                                                                                                                                                                144,
                                                                                                                                                            ],
                                                                                                                                                        ),
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 577,
                                                                                                                                                        end: 643,
                                                                                                                                                        as_str(): "0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 558,
                                                                                                                                        end: 643,
                                                                                                                                        as_str(): "keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ],
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 556,
                                                                                                                        end: 644,
                                                                                                                        as_str(): "!(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90)",
                                                                                                                    },
                                                                                                                },
                                                                                                                then: Expression {
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
                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 655,
                                                                                                                                                end: 656,
                                                                                                                                                as_str(): "4",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 655,
                                                                                                                                        end: 656,
                                                                                                                                        as_str(): "4",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ],
                                                                                                                            whole_block_span: Span {
                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 645,
                                                                                                                                end: 662,
                                                                                                                                as_str(): "{\n        4\n    }",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 645,
                                                                                                                        end: 662,
                                                                                                                        as_str(): "{\n        4\n    }",
                                                                                                                    },
                                                                                                                },
                                                                                                                else: Some(
                                                                                                                    Expression {
                                                                                                                        kind: If(
                                                                                                                            IfExpression {
                                                                                                                                condition: Expression {
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
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 671,
                                                                                                                                                                    end: 672,
                                                                                                                                                                    as_str(): "!",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                    "ops",
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 671,
                                                                                                                                                                    end: 672,
                                                                                                                                                                    as_str(): "!",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                        ],
                                                                                                                                                        suffix: BaseIdent {
                                                                                                                                                            name_override_opt: Some(
                                                                                                                                                                "not",
                                                                                                                                                            ),
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 671,
                                                                                                                                                                end: 672,
                                                                                                                                                                as_str(): "!",
                                                                                                                                                            },
                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                        },
                                                                                                                                                        is_absolute: true,
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                type_arguments: [],
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 671,
                                                                                                                                                    end: 672,
                                                                                                                                                    as_str(): "!",
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
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 694,
                                                                                                                                                                                    end: 696,
                                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                                },
                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                            },
                                                                                                                                                                            BaseIdent {
                                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                                    "ops",
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 694,
                                                                                                                                                                                    end: 696,
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
                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 694,
                                                                                                                                                                                end: 696,
                                                                                                                                                                                as_str(): "==",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                        is_absolute: true,
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                type_arguments: [],
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 694,
                                                                                                                                                                    end: 696,
                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            contract_call_params: [],
                                                                                                                                                            arguments: [
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: FunctionApplication(
                                                                                                                                                                        FunctionApplicationExpression {
                                                                                                                                                                            call_path_binding: TypeBinding {
                                                                                                                                                                                inner: CallPath {
                                                                                                                                                                                    prefixes: [],
                                                                                                                                                                                    suffix: BaseIdent {
                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 673,
                                                                                                                                                                                            end: 679,
                                                                                                                                                                                            as_str(): "sha256",
                                                                                                                                                                                        },
                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                    },
                                                                                                                                                                                    is_absolute: false,
                                                                                                                                                                                },
                                                                                                                                                                                type_arguments: [],
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 673,
                                                                                                                                                                                    end: 679,
                                                                                                                                                                                    as_str(): "sha256",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            arguments: [
                                                                                                                                                                                Expression {
                                                                                                                                                                                    kind: Tuple(
                                                                                                                                                                                        [
                                                                                                                                                                                            Expression {
                                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                                        span: Span {
                                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                            ),
                                                                                                                                                                                                            start: 681,
                                                                                                                                                                                                            end: 685,
                                                                                                                                                                                                            as_str(): "aaaa",
                                                                                                                                                                                                        },
                                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                                    },
                                                                                                                                                                                                ),
                                                                                                                                                                                                span: Span {
                                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 681,
                                                                                                                                                                                                    end: 685,
                                                                                                                                                                                                    as_str(): "aaaa",
                                                                                                                                                                                                },
                                                                                                                                                                                            },
                                                                                                                                                                                            Expression {
                                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                                        span: Span {
                                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                            ),
                                                                                                                                                                                                            start: 687,
                                                                                                                                                                                                            end: 691,
                                                                                                                                                                                                            as_str(): "abaa",
                                                                                                                                                                                                        },
                                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                                    },
                                                                                                                                                                                                ),
                                                                                                                                                                                                span: Span {
                                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 687,
                                                                                                                                                                                                    end: 691,
                                                                                                                                                                                                    as_str(): "abaa",
                                                                                                                                                                                                },
                                                                                                                                                                                            },
                                                                                                                                                                                        ],
                                                                                                                                                                                    ),
                                                                                                                                                                                    span: Span {
                                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                        path: Some(
                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                        ),
                                                                                                                                                                                        start: 680,
                                                                                                                                                                                        end: 692,
                                                                                                                                                                                        as_str(): "(aaaa, abaa)",
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                            ],
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 673,
                                                                                                                                                                        end: 693,
                                                                                                                                                                        as_str(): "sha256((aaaa, abaa))",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: Literal(
                                                                                                                                                                        B256(
                                                                                                                                                                            [
                                                                                                                                                                                164,
                                                                                                                                                                                188,
                                                                                                                                                                                168,
                                                                                                                                                                                235,
                                                                                                                                                                                143,
                                                                                                                                                                                51,
                                                                                                                                                                                143,
                                                                                                                                                                                127,
                                                                                                                                                                                218,
                                                                                                                                                                                38,
                                                                                                                                                                                150,
                                                                                                                                                                                15,
                                                                                                                                                                                164,
                                                                                                                                                                                59,
                                                                                                                                                                                254,
                                                                                                                                                                                52,
                                                                                                                                                                                251,
                                                                                                                                                                                197,
                                                                                                                                                                                98,
                                                                                                                                                                                226,
                                                                                                                                                                                238,
                                                                                                                                                                                13,
                                                                                                                                                                                124,
                                                                                                                                                                                110,
                                                                                                                                                                                136,
                                                                                                                                                                                86,
                                                                                                                                                                                193,
                                                                                                                                                                                197,
                                                                                                                                                                                135,
                                                                                                                                                                                242,
                                                                                                                                                                                21,
                                                                                                                                                                                206,
                                                                                                                                                                            ],
                                                                                                                                                                        ),
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 697,
                                                                                                                                                                        end: 763,
                                                                                                                                                                        as_str(): "0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 673,
                                                                                                                                                        end: 763,
                                                                                                                                                        as_str(): "sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 671,
                                                                                                                                        end: 764,
                                                                                                                                        as_str(): "!(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce)",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                then: Expression {
                                                                                                                                    kind: CodeBlock(
                                                                                                                                        CodeBlock {
                                                                                                                                            contents: [
                                                                                                                                                AstNode {
                                                                                                                                                    content: ImplicitReturnExpression(
                                                                                                                                                        Expression {
                                                                                                                                                            kind: Literal(
                                                                                                                                                                Numeric(
                                                                                                                                                                    5,
                                                                                                                                                                ),
                                                                                                                                                            ),
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 775,
                                                                                                                                                                end: 776,
                                                                                                                                                                as_str(): "5",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 775,
                                                                                                                                                        end: 776,
                                                                                                                                                        as_str(): "5",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                            whole_block_span: Span {
                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 765,
                                                                                                                                                end: 782,
                                                                                                                                                as_str(): "{\n        5\n    }",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 765,
                                                                                                                                        end: 782,
                                                                                                                                        as_str(): "{\n        5\n    }",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                else: Some(
                                                                                                                                    Expression {
                                                                                                                                        kind: If(
                                                                                                                                            IfExpression {
                                                                                                                                                condition: Expression {
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
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 791,
                                                                                                                                                                                    end: 792,
                                                                                                                                                                                    as_str(): "!",
                                                                                                                                                                                },
                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                            },
                                                                                                                                                                            BaseIdent {
                                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                                    "ops",
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 791,
                                                                                                                                                                                    end: 792,
                                                                                                                                                                                    as_str(): "!",
                                                                                                                                                                                },
                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                            },
                                                                                                                                                                        ],
                                                                                                                                                                        suffix: BaseIdent {
                                                                                                                                                                            name_override_opt: Some(
                                                                                                                                                                                "not",
                                                                                                                                                                            ),
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 791,
                                                                                                                                                                                end: 792,
                                                                                                                                                                                as_str(): "!",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                        is_absolute: true,
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                type_arguments: [],
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 791,
                                                                                                                                                                    end: 792,
                                                                                                                                                                    as_str(): "!",
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
                                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 817,
                                                                                                                                                                                                    end: 819,
                                                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                                                },
                                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                                            },
                                                                                                                                                                                            BaseIdent {
                                                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                                                    "ops",
                                                                                                                                                                                                ),
                                                                                                                                                                                                span: Span {
                                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 817,
                                                                                                                                                                                                    end: 819,
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
                                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                path: Some(
                                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                ),
                                                                                                                                                                                                start: 817,
                                                                                                                                                                                                end: 819,
                                                                                                                                                                                                as_str(): "==",
                                                                                                                                                                                            },
                                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                                        },
                                                                                                                                                                                        is_absolute: true,
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                                type_arguments: [],
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 817,
                                                                                                                                                                                    end: 819,
                                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            contract_call_params: [],
                                                                                                                                                                            arguments: [
                                                                                                                                                                                Expression {
                                                                                                                                                                                    kind: FunctionApplication(
                                                                                                                                                                                        FunctionApplicationExpression {
                                                                                                                                                                                            call_path_binding: TypeBinding {
                                                                                                                                                                                                inner: CallPath {
                                                                                                                                                                                                    prefixes: [],
                                                                                                                                                                                                    suffix: BaseIdent {
                                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                                        span: Span {
                                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                            ),
                                                                                                                                                                                                            start: 793,
                                                                                                                                                                                                            end: 802,
                                                                                                                                                                                                            as_str(): "keccak256",
                                                                                                                                                                                                        },
                                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                                    },
                                                                                                                                                                                                    is_absolute: false,
                                                                                                                                                                                                },
                                                                                                                                                                                                type_arguments: [],
                                                                                                                                                                                                span: Span {
                                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 793,
                                                                                                                                                                                                    end: 802,
                                                                                                                                                                                                    as_str(): "keccak256",
                                                                                                                                                                                                },
                                                                                                                                                                                            },
                                                                                                                                                                                            arguments: [
                                                                                                                                                                                                Expression {
                                                                                                                                                                                                    kind: Tuple(
                                                                                                                                                                                                        [
                                                                                                                                                                                                            Expression {
                                                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                                                        span: Span {
                                                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                                            ),
                                                                                                                                                                                                                            start: 804,
                                                                                                                                                                                                                            end: 808,
                                                                                                                                                                                                                            as_str(): "aaaa",
                                                                                                                                                                                                                        },
                                                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                                                    },
                                                                                                                                                                                                                ),
                                                                                                                                                                                                                span: Span {
                                                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                                    ),
                                                                                                                                                                                                                    start: 804,
                                                                                                                                                                                                                    end: 808,
                                                                                                                                                                                                                    as_str(): "aaaa",
                                                                                                                                                                                                                },
                                                                                                                                                                                                            },
                                                                                                                                                                                                            Expression {
                                                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                                                        span: Span {
                                                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                                            ),
                                                                                                                                                                                                                            start: 810,
                                                                                                                                                                                                                            end: 814,
                                                                                                                                                                                                                            as_str(): "abaa",
                                                                                                                                                                                                                        },
                                                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                                                    },
                                                                                                                                                                                                                ),
                                                                                                                                                                                                                span: Span {
                                                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                                    ),
                                                                                                                                                                                                                    start: 810,
                                                                                                                                                                                                                    end: 814,
                                                                                                                                                                                                                    as_str(): "abaa",
                                                                                                                                                                                                                },
                                                                                                                                                                                                            },
                                                                                                                                                                                                        ],
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    span: Span {
                                                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                        path: Some(
                                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                        ),
                                                                                                                                                                                                        start: 803,
                                                                                                                                                                                                        end: 815,
                                                                                                                                                                                                        as_str(): "(aaaa, abaa)",
                                                                                                                                                                                                    },
                                                                                                                                                                                                },
                                                                                                                                                                                            ],
                                                                                                                                                                                        },
                                                                                                                                                                                    ),
                                                                                                                                                                                    span: Span {
                                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                        path: Some(
                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                        ),
                                                                                                                                                                                        start: 793,
                                                                                                                                                                                        end: 816,
                                                                                                                                                                                        as_str(): "keccak256((aaaa, abaa))",
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                                Expression {
                                                                                                                                                                                    kind: Literal(
                                                                                                                                                                                        B256(
                                                                                                                                                                                            [
                                                                                                                                                                                                79,
                                                                                                                                                                                                206,
                                                                                                                                                                                                90,
                                                                                                                                                                                                41,
                                                                                                                                                                                                112,
                                                                                                                                                                                                64,
                                                                                                                                                                                                216,
                                                                                                                                                                                                46,
                                                                                                                                                                                                236,
                                                                                                                                                                                                247,
                                                                                                                                                                                                176,
                                                                                                                                                                                                174,
                                                                                                                                                                                                72,
                                                                                                                                                                                                85,
                                                                                                                                                                                                173,
                                                                                                                                                                                                67,
                                                                                                                                                                                                105,
                                                                                                                                                                                                143,
                                                                                                                                                                                                25,
                                                                                                                                                                                                30,
                                                                                                                                                                                                227,
                                                                                                                                                                                                136,
                                                                                                                                                                                                32,
                                                                                                                                                                                                226,
                                                                                                                                                                                                119,
                                                                                                                                                                                                72,
                                                                                                                                                                                                100,
                                                                                                                                                                                                135,
                                                                                                                                                                                                101,
                                                                                                                                                                                                188,
                                                                                                                                                                                                66,
                                                                                                                                                                                                189,
                                                                                                                                                                                            ],
                                                                                                                                                                                        ),
                                                                                                                                                                                    ),
                                                                                                                                                                                    span: Span {
                                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                        path: Some(
                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                        ),
                                                                                                                                                                                        start: 820,
                                                                                                                                                                                        end: 886,
                                                                                                                                                                                        as_str(): "0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd",
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                            ],
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 793,
                                                                                                                                                                        end: 886,
                                                                                                                                                                        as_str(): "keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 791,
                                                                                                                                                        end: 887,
                                                                                                                                                        as_str(): "!(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd)",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                then: Expression {
                                                                                                                                                    kind: CodeBlock(
                                                                                                                                                        CodeBlock {
                                                                                                                                                            contents: [
                                                                                                                                                                AstNode {
                                                                                                                                                                    content: ImplicitReturnExpression(
                                                                                                                                                                        Expression {
                                                                                                                                                                            kind: Literal(
                                                                                                                                                                                Numeric(
                                                                                                                                                                                    6,
                                                                                                                                                                                ),
                                                                                                                                                                            ),
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 898,
                                                                                                                                                                                end: 899,
                                                                                                                                                                                as_str(): "6",
                                                                                                                                                                            },
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 898,
                                                                                                                                                                        end: 899,
                                                                                                                                                                        as_str(): "6",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                            whole_block_span: Span {
                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 888,
                                                                                                                                                                end: 905,
                                                                                                                                                                as_str(): "{\n        6\n    }",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 888,
                                                                                                                                                        end: 905,
                                                                                                                                                        as_str(): "{\n        6\n    }",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                else: Some(
                                                                                                                                                    Expression {
                                                                                                                                                        kind: CodeBlock(
                                                                                                                                                            CodeBlock {
                                                                                                                                                                contents: [
                                                                                                                                                                    AstNode {
                                                                                                                                                                        content: ImplicitReturnExpression(
                                                                                                                                                                            Expression {
                                                                                                                                                                                kind: Literal(
                                                                                                                                                                                    Numeric(
                                                                                                                                                                                        100,
                                                                                                                                                                                    ),
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 921,
                                                                                                                                                                                    end: 924,
                                                                                                                                                                                    as_str(): "100",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                        ),
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 921,
                                                                                                                                                                            end: 924,
                                                                                                                                                                            as_str(): "100",
                                                                                                                                                                        },
                                                                                                                                                                    },
                                                                                                                                                                ],
                                                                                                                                                                whole_block_span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 911,
                                                                                                                                                                    end: 930,
                                                                                                                                                                    as_str(): "{\n        100\n    }",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 911,
                                                                                                                                                            end: 930,
                                                                                                                                                            as_str(): "{\n        100\n    }",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ),
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 788,
                                                                                                                                            end: 930,
                                                                                                                                            as_str(): "if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 668,
                                                                                                                            end: 930,
                                                                                                                            as_str(): "if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 553,
                                                                                                            end: 930,
                                                                                                            as_str(): "if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                            ),
                                                                                            start: 441,
                                                                                            end: 930,
                                                                                            as_str(): "if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                            ),
                                                                            start: 399,
                                                                            end: 930,
                                                                            as_str(): "if !(aaaa == aaaa) {\n        2\n    } else if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 360,
                                                            end: 930,
                                                            as_str(): "if aaaa == abaa {\n        1\n    } else if !(aaaa == aaaa) {\n        2\n    } else if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14cbeadf0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                            ),
                                            start: 321,
                                            end: 930,
                                            as_str(): "if aaaa == aaab {\n        0\n    } else if aaaa == abaa {\n        1\n    } else if !(aaaa == aaaa) {\n        2\n    } else if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14cbeadf0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                    ),
                                    start: 321,
                                    end: 930,
                                    as_str(): "if aaaa == aaab {\n        0\n    } else if aaaa == abaa {\n        1\n    } else if !(aaaa == aaaa) {\n        2\n    } else if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 63,
                            end: 932,
                            as_str(): "{\n    let aaaa = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    let aaab = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_b;\n    let abaa = 0xa_b_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if aaaa == aaab {\n        0\n    } else if aaaa == abaa {\n        1\n    } else if !(aaaa == aaaa) {\n        2\n    } else if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb14cbeadf0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                        ),
                        start: 46,
                        end: 932,
                        as_str(): "fn main() -> u64 {\n    let aaaa = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    let aaab = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_b;\n    let abaa = 0xa_b_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if aaaa == aaab {\n        0\n    } else if aaaa == abaa {\n        1\n    } else if !(aaaa == aaaa) {\n        2\n    } else if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb14cbeadf0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                        ),
                        start: 59,
                        end: 62,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb14cbeadf0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
            ),
            start: 46,
            end: 932,
            as_str(): "fn main() -> u64 {\n    let aaaa = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    let aaab = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_b;\n    let abaa = 0xa_b_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if aaaa == aaab {\n        0\n    } else if aaaa == abaa {\n        1\n    } else if !(aaaa == aaaa) {\n        2\n    } else if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }\n}",
        },
    },
]
