[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fb11f9dcb50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                    ),
                    start: 9,
                    end: 20,
                    as_str(): "dep heaven;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fb11f9dcb50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                    ),
                    start: 13,
                    end: 19,
                    as_str(): "heaven",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 9,
            end: 20,
            as_str(): "dep heaven;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fb11f9dcb50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                    ),
                    start: 21,
                    end: 31,
                    as_str(): "dep earth;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fb11f9dcb50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                    ),
                    start: 25,
                    end: 30,
                    as_str(): "earth",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 21,
            end: 31,
            as_str(): "dep earth;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fb11f9dcb50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                    ),
                    start: 32,
                    end: 41,
                    as_str(): "dep hell;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fb11f9dcb50,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                    ),
                    start: 36,
                    end: 40,
                    as_str(): "hell",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 32,
            end: 41,
            as_str(): "dep hell;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 47,
                            end: 53,
                            as_str(): "heaven",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 55,
                            end: 74,
                            as_str(): "UNKNOWN_DEITY_VALUE",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 43,
            end: 75,
            as_str(): "use heaven::UNKNOWN_DEITY_VALUE;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 80,
                            end: 85,
                            as_str(): "earth",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 87,
                            end: 90,
                            as_str(): "MAN",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 76,
            end: 91,
            as_str(): "use earth::MAN;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 96,
                            end: 100,
                            as_str(): "hell",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 102,
                            end: 111,
                            as_str(): "THE_DEVIL",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 92,
            end: 112,
            as_str(): "use hell::THE_DEVIL;",
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
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 117,
                            end: 120,
                            as_str(): "god",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
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
                                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                                ),
                                                                                                start: 143,
                                                                                                end: 145,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                                ),
                                                                                                start: 143,
                                                                                                end: 145,
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
                                                                                            src (ptr): 0x00007fb11f9dcb50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                            ),
                                                                                            start: 143,
                                                                                            end: 145,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                ),
                                                                                start: 143,
                                                                                end: 145,
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
                                                                                            src (ptr): 0x00007fb11f9dcb50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                            ),
                                                                                            start: 139,
                                                                                            end: 142,
                                                                                            as_str(): "MAN",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 139,
                                                                                    end: 142,
                                                                                    as_str(): "MAN",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 146,
                                                                                    end: 147,
                                                                                    as_str(): "5",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                    ),
                                                                    start: 139,
                                                                    end: 147,
                                                                    as_str(): "MAN == 5",
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
                                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                                ),
                                                                                                start: 161,
                                                                                                end: 163,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                                ),
                                                                                                start: 161,
                                                                                                end: 163,
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
                                                                                            src (ptr): 0x00007fb11f9dcb50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                            ),
                                                                                            start: 161,
                                                                                            end: 163,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: true,
                                                                                },
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                ),
                                                                                start: 161,
                                                                                end: 163,
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
                                                                                            src (ptr): 0x00007fb11f9dcb50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                            ),
                                                                                            start: 151,
                                                                                            end: 160,
                                                                                            as_str(): "THE_DEVIL",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 151,
                                                                                    end: 160,
                                                                                    as_str(): "THE_DEVIL",
                                                                                },
                                                                            },
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        6,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 164,
                                                                                    end: 165,
                                                                                    as_str(): "6",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                    ),
                                                                    start: 151,
                                                                    end: 165,
                                                                    as_str(): "THE_DEVIL == 6",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f9dcb50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                        ),
                                                        start: 139,
                                                        end: 165,
                                                        as_str(): "MAN == 5 && THE_DEVIL == 6",
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
                                                                                    7,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                ),
                                                                                start: 176,
                                                                                end: 177,
                                                                                as_str(): "7",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f9dcb50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                        ),
                                                                        start: 176,
                                                                        end: 177,
                                                                        as_str(): "7",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                ),
                                                                start: 166,
                                                                end: 183,
                                                                as_str(): "{\n        7\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f9dcb50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                        ),
                                                        start: 166,
                                                        end: 183,
                                                        as_str(): "{\n        7\n    }",
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
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f9dcb50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                            ),
                                                                                            start: 199,
                                                                                            end: 218,
                                                                                            as_str(): "UNKNOWN_DEITY_VALUE",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 199,
                                                                                    end: 218,
                                                                                    as_str(): "UNKNOWN_DEITY_VALUE",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f9dcb50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                            ),
                                                                            start: 199,
                                                                            end: 218,
                                                                            as_str(): "UNKNOWN_DEITY_VALUE",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                    ),
                                                                    start: 189,
                                                                    end: 224,
                                                                    as_str(): "{\n        UNKNOWN_DEITY_VALUE\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f9dcb50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                            ),
                                                            start: 189,
                                                            end: 224,
                                                            as_str(): "{\n        UNKNOWN_DEITY_VALUE\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f9dcb50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                            ),
                                            start: 136,
                                            end: 224,
                                            as_str(): "if MAN == 5 && THE_DEVIL == 6 {\n        7\n    } else {\n        UNKNOWN_DEITY_VALUE\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f9dcb50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                    ),
                                    start: 136,
                                    end: 224,
                                    as_str(): "if MAN == 5 && THE_DEVIL == 6 {\n        7\n    } else {\n        UNKNOWN_DEITY_VALUE\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 130,
                            end: 226,
                            as_str(): "{\n    if MAN == 5 && THE_DEVIL == 6 {\n        7\n    } else {\n        UNKNOWN_DEITY_VALUE\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb11f9dcb50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                        ),
                        start: 114,
                        end: 226,
                        as_str(): "fn god() -> u64 {\n    if MAN == 5 && THE_DEVIL == 6 {\n        7\n    } else {\n        UNKNOWN_DEITY_VALUE\n    }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb11f9dcb50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                        ),
                        start: 126,
                        end: 129,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 114,
            end: 226,
            as_str(): "fn god() -> u64 {\n    if MAN == 5 && THE_DEVIL == 6 {\n        7\n    } else {\n        UNKNOWN_DEITY_VALUE\n    }\n}",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 232,
                            end: 238,
                            as_str(): "heaven",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 240,
                            end: 257,
                            as_str(): "MONKEYS_GONE_HERE",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 228,
            end: 258,
            as_str(): "use heaven::MONKEYS_GONE_HERE;",
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
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 263,
                            end: 267,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 293,
                                                                                    end: 295,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 293,
                                                                                    end: 295,
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
                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                ),
                                                                                start: 293,
                                                                                end: 295,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                    ),
                                                                    start: 293,
                                                                    end: 295,
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
                                                                                            src (ptr): 0x00007fb11f9dcb50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                            ),
                                                                                            start: 287,
                                                                                            end: 290,
                                                                                            as_str(): "god",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 287,
                                                                                    end: 290,
                                                                                    as_str(): "god",
                                                                                },
                                                                            },
                                                                            arguments: [],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f9dcb50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                        ),
                                                                        start: 287,
                                                                        end: 292,
                                                                        as_str(): "god()",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            7,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f9dcb50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                        ),
                                                                        start: 296,
                                                                        end: 297,
                                                                        as_str(): "7",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f9dcb50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                        ),
                                                        start: 287,
                                                        end: 297,
                                                        as_str(): "god() == 7",
                                                    },
                                                },
                                                then: Expression {
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
                                                                                        src (ptr): 0x00007fb11f9dcb50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                        ),
                                                                                        start: 308,
                                                                                        end: 325,
                                                                                        as_str(): "MONKEYS_GONE_HERE",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                ),
                                                                                start: 308,
                                                                                end: 325,
                                                                                as_str(): "MONKEYS_GONE_HERE",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f9dcb50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                        ),
                                                                        start: 308,
                                                                        end: 325,
                                                                        as_str(): "MONKEYS_GONE_HERE",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                ),
                                                                start: 298,
                                                                end: 331,
                                                                as_str(): "{\n        MONKEYS_GONE_HERE\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f9dcb50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                        ),
                                                        start: 298,
                                                        end: 331,
                                                        as_str(): "{\n        MONKEYS_GONE_HERE\n    }",
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
                                                                                    Boolean(
                                                                                        false,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                                    ),
                                                                                    start: 347,
                                                                                    end: 352,
                                                                                    as_str(): "false",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f9dcb50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                            ),
                                                                            start: 347,
                                                                            end: 352,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                    ),
                                                                    start: 337,
                                                                    end: 358,
                                                                    as_str(): "{\n        false\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f9dcb50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                            ),
                                                            start: 337,
                                                            end: 358,
                                                            as_str(): "{\n        false\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f9dcb50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                            ),
                                            start: 284,
                                            end: 358,
                                            as_str(): "if god() == 7 {\n        MONKEYS_GONE_HERE\n    } else {\n        false\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f9dcb50,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                    ),
                                    start: 284,
                                    end: 358,
                                    as_str(): "if god() == 7 {\n        MONKEYS_GONE_HERE\n    } else {\n        false\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb11f9dcb50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                            ),
                            start: 278,
                            end: 360,
                            as_str(): "{\n    if god() == 7 {\n        MONKEYS_GONE_HERE\n    } else {\n        false\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb11f9dcb50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                        ),
                        start: 260,
                        end: 360,
                        as_str(): "fn main() -> bool {\n    if god() == 7 {\n        MONKEYS_GONE_HERE\n    } else {\n        false\n    }\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb11f9dcb50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                        ),
                        start: 273,
                        end: 277,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f9dcb50,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
            ),
            start: 260,
            end: 360,
            as_str(): "fn main() -> bool {\n    if god() == 7 {\n        MONKEYS_GONE_HERE\n    } else {\n        false\n    }\n}",
        },
    },
]
