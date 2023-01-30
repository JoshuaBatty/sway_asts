[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb10d4510c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                src (ptr): 0x00007fb10d4510c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                ),
                                start: 35,
                                end: 36,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb10d4510c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                        src (ptr): 0x00007fb10d4510c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                src (ptr): 0x00007fb10d4510c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                ),
                                start: 45,
                                end: 46,
                                as_str(): "E",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb10d4510c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                ),
                                start: 40,
                                end: 46,
                                as_str(): "Err: E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb10d4510c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
            src (ptr): 0x00007fb10d4510c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                            src (ptr): 0x00007fb10d4510c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
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
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 78,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d4510c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                        ),
                                                        start: 80,
                                                        end: 86,
                                                        as_str(): "Result",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                21,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 87,
                                                                end: 90,
                                                                as_str(): "u64",
                                                            },
                                                        },
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                21,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 95,
                                                                as_str(): "u64",
                                                            },
                                                        },
                                                    ],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 80,
                                                    end: 96,
                                                    as_str(): "Result<u64, u64>",
                                                },
                                            ),
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
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 99,
                                                                                end: 105,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 99,
                                                                            end: 105,
                                                                            as_str(): "Result",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 107,
                                                                            end: 109,
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
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 112,
                                                                        end: 115,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                                TypeArgument {
                                                                    type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    initial_type_id: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 120,
                                                                        as_str(): "u64",
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 99,
                                                                end: 121,
                                                                as_str(): "Result::Ok::<u64, u64>",
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
                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 122,
                                                                    end: 126,
                                                                    as_str(): "5u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb10d4510c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                    ),
                                                    start: 99,
                                                    end: 127,
                                                    as_str(): "Result::Ok::<u64, u64>(5u64)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                    ),
                                    start: 73,
                                    end: 128,
                                    as_str(): "let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);",
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
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 181,
                                                                end: 182,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb10d4510c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                        ),
                                                        start: 181,
                                                        end: 182,
                                                        as_str(): "x",
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
                                                                            src (ptr): 0x00007fb10d4510c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 165,
                                                                            end: 171,
                                                                            as_str(): "Result",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 173,
                                                                        end: 175,
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
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 176,
                                                                        end: 177,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 176,
                                                                    end: 177,
                                                                    as_str(): "y",
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 165,
                                                                end: 178,
                                                                as_str(): "Result::Ok(y)",
                                                            },
                                                        },
                                                        result: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
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
                                                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 187,
                                                                                                                    end: 188,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 187,
                                                                                                                    end: 188,
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
                                                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 187,
                                                                                                                end: 188,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: true,
                                                                                                    },
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb10d4510c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 187,
                                                                                                    end: 188,
                                                                                                    as_str(): "+",
                                                                                                },
                                                                                            },
                                                                                            contract_call_params: [],
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 185,
                                                                                                                end: 186,
                                                                                                                as_str(): "y",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 185,
                                                                                                        end: 186,
                                                                                                        as_str(): "y",
                                                                                                    },
                                                                                                },
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            10,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 189,
                                                                                                        end: 191,
                                                                                                        as_str(): "10",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 185,
                                                                                        end: 191,
                                                                                        as_str(): "y + 10",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 185,
                                                                                end: 191,
                                                                                as_str(): "y + 10",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 183,
                                                                        end: 193,
                                                                        as_str(): "{ y + 10 }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 183,
                                                                end: 193,
                                                                as_str(): "{ y + 10 }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 165,
                                                            end: 193,
                                                            as_str(): "Result::Ok(y) = x { y + 10 }",
                                                        },
                                                    },
                                                    MatchBranch {
                                                        scrutinee: CatchAll {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 199,
                                                                end: 204,
                                                                as_str(): "{ 1 }",
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
                                                                                            1,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 201,
                                                                                        end: 202,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10d4510c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 201,
                                                                                end: 202,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb10d4510c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 199,
                                                                        end: 204,
                                                                        as_str(): "{ 1 }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb10d4510c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                                ),
                                                                start: 199,
                                                                end: 204,
                                                                as_str(): "{ 1 }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10d4510c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 204,
                                                            as_str(): "{ 1 }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb10d4510c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                            ),
                                            start: 158,
                                            end: 204,
                                            as_str(): "if let Result::Ok(y) = x { y + 10 } else { 1 }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb10d4510c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                                    ),
                                    start: 158,
                                    end: 204,
                                    as_str(): "if let Result::Ok(y) = x { y + 10 } else { 1 }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb10d4510c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                            ),
                            start: 68,
                            end: 206,
                            as_str(): "{\n   let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    // should return 15\n    if let Result::Ok(y) = x { y + 10 } else { 1 }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb10d4510c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                        ),
                        start: 51,
                        end: 206,
                        as_str(): "fn main() -> u64 {\n   let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    // should return 15\n    if let Result::Ok(y) = x { y + 10 } else { 1 }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb10d4510c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
                        ),
                        start: 64,
                        end: 67,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb10d4510c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR69wzgA/enum_destructuring/src/main.sw",
            ),
            start: 51,
            end: 206,
            as_str(): "fn main() -> u64 {\n   let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    // should return 15\n    if let Result::Ok(y) = x { y + 10 } else { 1 }\n}",
        },
    },
]
