






TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: LazyOperator {
                                    op: And,
                                    lhs: TyExpression {
                                        expression: FunctionApplication {
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
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12bd9fa00,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12736cfc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/earth.sw",
                                                                    ),
                                                                    start: 26,
                                                                    end: 29,
                                                                    as_str(): "MAN",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                ),
                                                                start: 139,
                                                                end: 142,
                                                                as_str(): "MAN",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
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
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12bd9fa00,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                5,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
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
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                548,
                                                Span {
                                                    src (ptr): 0x00007fb12bd9fa00,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
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
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
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
                                    rhs: TyExpression {
                                        expression: FunctionApplication {
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
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12bd9fa00,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b8af290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/hell.sw",
                                                                    ),
                                                                    start: 25,
                                                                    end: 34,
                                                                    as_str(): "THE_DEVIL",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                ),
                                                                start: 151,
                                                                end: 160,
                                                                as_str(): "THE_DEVIL",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            21,
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
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb12bd9fa00,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                6,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
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
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                549,
                                                Span {
                                                    src (ptr): 0x00007fb12bd9fa00,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
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
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
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
                                return_type: TypeId(
                                    71,
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
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                7,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
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
                                    },
                                ),
                                return_type: TypeId(
                                    21,
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
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: VariableExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11fd04560,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/heaven.sw",
                                                                        ),
                                                                        start: 27,
                                                                        end: 46,
                                                                        as_str(): "UNKNOWN_DEITY_VALUE",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f9dcb50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                    ),
                                                                    start: 199,
                                                                    end: 218,
                                                                    as_str(): "UNKNOWN_DEITY_VALUE",
                                                                },
                                                                mutability: Immutable,
                                                            },
                                                            return_type: TypeId(
                                                                21,
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
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
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
                        return_type: TypeId(
                            21,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb11f9dcb50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
        ),
        start: 114,
        end: 226,
        as_str(): "fn god() -> u64 {\n    if MAN == 5 && THE_DEVIL == 6 {\n        7\n    } else {\n        UNKNOWN_DEITY_VALUE\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
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
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12bd9fa00,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3022,
                                                    end: 3026,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
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
                                                    contract_call_params: {},
                                                    arguments: [],
                                                    function_decl_id: DeclId(
                                                        552,
                                                        Span {
                                                            src (ptr): 0x00007fb11f9dcb50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 226,
                                                            as_str(): "fn god() -> u64 {\n    if MAN == 5 && THE_DEVIL == 6 {\n        7\n    } else {\n        UNKNOWN_DEITY_VALUE\n    }\n}",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
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
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
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
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12bd9fa00,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3028,
                                                    end: 3033,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        7,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
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
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        553,
                                        Span {
                                            src (ptr): 0x00007fb12bd9fa00,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3016,
                                            end: 3082,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
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
                                    ),
                                },
                                return_type: TypeId(
                                    71,
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
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11fd04560,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/heaven.sw",
                                                                    ),
                                                                    start: 62,
                                                                    end: 79,
                                                                    as_str(): "MONKEYS_GONE_HERE",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f9dcb50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
                                                                ),
                                                                start: 308,
                                                                end: 325,
                                                                as_str(): "MONKEYS_GONE_HERE",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            71,
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
                                    },
                                ),
                                return_type: TypeId(
                                    71,
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
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                Boolean(
                                                                    false,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                71,
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
                                        },
                                    ),
                                    return_type: TypeId(
                                        71,
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
                        return_type: TypeId(
                            71,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb11f9dcb50,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnHMDBg/const_decl_in_library/src/main.sw",
        ),
        start: 260,
        end: 360,
        as_str(): "fn main() -> bool {\n    if god() == 7 {\n        MONKEYS_GONE_HERE\n    } else {\n        false\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

