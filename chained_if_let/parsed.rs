[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12e8db920,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                            ),
                            start: 14,
                            end: 20,
                            as_str(): "Result",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(7249),
                        E: TypeId(7250),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 31,
                                    end: 33,
                                    as_str(): "Ok",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 36,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb12e8db920,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                ),
                                start: 35,
                                end: 36,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb12e8db920,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                ),
                                start: 31,
                                end: 36,
                                as_str(): "Ok: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 40,
                                    end: 43,
                                    as_str(): "Err",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb12e8db920,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 46,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb12e8db920,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                ),
                                start: 45,
                                end: 46,
                                as_str(): "E",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb12e8db920,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                ),
                                start: 40,
                                end: 46,
                                as_str(): "Err: E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb12e8db920,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                        ),
                        start: 9,
                        end: 49,
                        as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12e8db920,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
            ),
            start: 9,
            end: 49,
            as_str(): "enum Result<T, E> {\n  Ok: T,\n  Err: E,\n}",
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
                            src (ptr): 0x00007fb12e8db920,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                            ),
                            start: 73,
                            end: 77,
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
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 95,
                                                    end: 103,
                                                    as_str(): "result_a",
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
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 106,
                                                                                end: 112,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 106,
                                                                            end: 112,
                                                                            as_str(): "Result",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 114,
                                                                            end: 116,
                                                                            as_str(): "Ok",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 119,
                                                                        end: 122,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        71,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 124,
                                                                        end: 128,
                                                                        as_str(): "bool",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 129,
                                                                as_str(): "Result::Ok::<u64, bool>",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U64(
                                                                        5,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12e8db920,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                    ),
                                                                    start: 130,
                                                                    end: 134,
                                                                    as_str(): "5u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 106,
                                                    end: 135,
                                                    as_str(): "Result::Ok::<u64, bool>(5u64)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 91,
                                    end: 136,
                                    as_str(): "let result_a = Result::Ok::<u64, bool>(5u64);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 151,
                                                    as_str(): "result_b",
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
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 154,
                                                                                end: 160,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 154,
                                                                            end: 160,
                                                                            as_str(): "Result",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 162,
                                                                            end: 165,
                                                                            as_str(): "Err",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 168,
                                                                        end: 171,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        71,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        71,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 173,
                                                                        end: 177,
                                                                        as_str(): "bool",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 154,
                                                                end: 178,
                                                                as_str(): "Result::Err::<u64, bool>",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Boolean(
                                                                        false,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12e8db920,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                    ),
                                                                    start: 179,
                                                                    end: 184,
                                                                    as_str(): "false",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8db920,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                    ),
                                                    start: 154,
                                                    end: 185,
                                                    as_str(): "Result::Err::<u64, bool>(false)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 139,
                                    end: 186,
                                    as_str(): "let result_b = Result::Err::<u64, bool>(false);",
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
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 214,
                                                                end: 222,
                                                                as_str(): "result_a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb12e8db920,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                        ),
                                                        start: 214,
                                                        end: 222,
                                                        as_str(): "result_a",
                                                    },
                                                },
                                                branches: [
                                                    MatchBranch {
                                                        scrutinee: EnumScrutinee {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 197,
                                                                            end: 203,
                                                                            as_str(): "Result",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 205,
                                                                        end: 208,
                                                                        as_str(): "Err",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            value: Variable {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 209,
                                                                        end: 210,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12e8db920,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                    ),
                                                                    start: 209,
                                                                    end: 210,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 197,
                                                                end: 211,
                                                                as_str(): "Result::Err(a)",
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
                                                                                            6,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 229,
                                                                                        end: 230,
                                                                                        as_str(): "6",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 229,
                                                                                end: 230,
                                                                                as_str(): "6",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb12e8db920,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                        ),
                                                                        start: 223,
                                                                        end: 234,
                                                                        as_str(): "{\n    6\n  }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 223,
                                                                end: 234,
                                                                as_str(): "{\n    6\n  }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 197,
                                                            end: 234,
                                                            as_str(): "Result::Err(a) = result_a {\n    6\n  }",
                                                        },
                                                    },
                                                    MatchBranch {
                                                        scrutinee: CatchAll {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 240,
                                                                end: 359,
                                                                as_str(): "if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                                                            },
                                                        },
                                                        result: Expression {
                                                            kind: Match(
                                                                MatchExpression {
                                                                    value: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 265,
                                                                                    end: 273,
                                                                                    as_str(): "result_b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12e8db920,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 273,
                                                                            as_str(): "result_b",
                                                                        },
                                                                    },
                                                                    branches: [
                                                                        MatchBranch {
                                                                            scrutinee: EnumScrutinee {
                                                                                call_path: CallPath {
                                                                                    prefixes: [
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 247,
                                                                                                end: 253,
                                                                                                as_str(): "Result",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 255,
                                                                                            end: 257,
                                                                                            as_str(): "Ok",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                value: Variable {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 258,
                                                                                            end: 261,
                                                                                            as_str(): "num",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 258,
                                                                                        end: 261,
                                                                                        as_str(): "num",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 247,
                                                                                    end: 262,
                                                                                    as_str(): "Result::Ok(num)",
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
                                                                                                                10,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 280,
                                                                                                            end: 282,
                                                                                                            as_str(): "10",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 280,
                                                                                                    end: 282,
                                                                                                    as_str(): "10",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 274,
                                                                                            end: 286,
                                                                                            as_str(): "{\n    10\n  }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 274,
                                                                                    end: 286,
                                                                                    as_str(): "{\n    10\n  }",
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 247,
                                                                                end: 286,
                                                                                as_str(): "Result::Ok(num) = result_b {\n    10\n  }",
                                                                            },
                                                                        },
                                                                        MatchBranch {
                                                                            scrutinee: CatchAll {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 292,
                                                                                    end: 359,
                                                                                    as_str(): "if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                                                                                },
                                                                            },
                                                                            result: Expression {
                                                                                kind: Match(
                                                                                    MatchExpression {
                                                                                        value: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 317,
                                                                                                        end: 325,
                                                                                                        as_str(): "result_a",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 317,
                                                                                                end: 325,
                                                                                                as_str(): "result_a",
                                                                                            },
                                                                                        },
                                                                                        branches: [
                                                                                            MatchBranch {
                                                                                                scrutinee: EnumScrutinee {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 299,
                                                                                                                    end: 305,
                                                                                                                    as_str(): "Result",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 307,
                                                                                                                end: 309,
                                                                                                                as_str(): "Ok",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: Variable {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 310,
                                                                                                                end: 313,
                                                                                                                as_str(): "num",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12e8db920,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 310,
                                                                                                            end: 313,
                                                                                                            as_str(): "num",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 299,
                                                                                                        end: 314,
                                                                                                        as_str(): "Result::Ok(num)",
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
                                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 332,
                                                                                                                                        end: 335,
                                                                                                                                        as_str(): "num",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 332,
                                                                                                                                end: 335,
                                                                                                                                as_str(): "num",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 332,
                                                                                                                        end: 335,
                                                                                                                        as_str(): "num",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                            whole_block_span: Span {
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 326,
                                                                                                                end: 339,
                                                                                                                as_str(): "{\n    num\n  }",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 326,
                                                                                                        end: 339,
                                                                                                        as_str(): "{\n    num\n  }",
                                                                                                    },
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 299,
                                                                                                    end: 339,
                                                                                                    as_str(): "Result::Ok(num) = result_a {\n    num\n  }",
                                                                                                },
                                                                                            },
                                                                                            MatchBranch {
                                                                                                scrutinee: CatchAll {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 345,
                                                                                                        end: 359,
                                                                                                        as_str(): "{ \n    42 \n  }",
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
                                                                                                                                    42,
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 352,
                                                                                                                                end: 354,
                                                                                                                                as_str(): "42",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 352,
                                                                                                                        end: 354,
                                                                                                                        as_str(): "42",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                            whole_block_span: Span {
                                                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 345,
                                                                                                                end: 359,
                                                                                                                as_str(): "{ \n    42 \n  }",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12e8db920,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 345,
                                                                                                        end: 359,
                                                                                                        as_str(): "{ \n    42 \n  }",
                                                                                                    },
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 345,
                                                                                                    end: 359,
                                                                                                    as_str(): "{ \n    42 \n  }",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12e8db920,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 292,
                                                                                    end: 359,
                                                                                    as_str(): "if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12e8db920,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                                ),
                                                                                start: 292,
                                                                                end: 359,
                                                                                as_str(): "if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12e8db920,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                                ),
                                                                start: 240,
                                                                end: 359,
                                                                as_str(): "if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12e8db920,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                                            ),
                                                            start: 240,
                                                            end: 359,
                                                            as_str(): "if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12e8db920,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                            ),
                                            start: 190,
                                            end: 359,
                                            as_str(): "if let Result::Err(a) = result_a {\n    6\n  } else if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12e8db920,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                                    ),
                                    start: 190,
                                    end: 359,
                                    as_str(): "if let Result::Err(a) = result_a {\n    6\n  } else if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12e8db920,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                            ),
                            start: 87,
                            end: 361,
                            as_str(): "{\n  let result_a = Result::Ok::<u64, bool>(5u64);\n  let result_b = Result::Err::<u64, bool>(false);\n\n  if let Result::Err(a) = result_a {\n    6\n  } else if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12e8db920,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                        ),
                        start: 70,
                        end: 361,
                        as_str(): "fn main() -> u64 {\n  let result_a = Result::Ok::<u64, bool>(5u64);\n  let result_b = Result::Err::<u64, bool>(false);\n\n  if let Result::Err(a) = result_a {\n    6\n  } else if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12e8db920,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
                        ),
                        start: 83,
                        end: 86,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12e8db920,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRTg21uo/chained_if_let/src/main.sw",
            ),
            start: 70,
            end: 361,
            as_str(): "fn main() -> u64 {\n  let result_a = Result::Ok::<u64, bool>(5u64);\n  let result_b = Result::Err::<u64, bool>(false);\n\n  if let Result::Err(a) = result_a {\n    6\n  } else if let Result::Ok(num) = result_b {\n    10\n  } else if let Result::Ok(num) = result_a {\n    num\n  } else { \n    42 \n  }\n}",
        },
    },
]
