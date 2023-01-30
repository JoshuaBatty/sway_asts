TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12d4a1aa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
            ),
            start: 15,
            end: 25,
            as_str(): "GLOBAL_VAL",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: Literal(
            U64(
                99,
            ),
        ),
        return_type: TypeId(
            7251,
        ),
        span: Span {
            src (ptr): 0x00007fb12d4a1aa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
            ),
            start: 33,
            end: 35,
            as_str(): "99",
        },
    },
    visibility: Private,
    return_type: TypeId(
        21,
    ),
    is_configurable: false,
    attributes: {},
    type_ascription_span: Some(
        Span {
            src (ptr): 0x00007fb12d4a1aa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
            ),
            start: 27,
            end: 30,
            as_str(): "u64",
        },
    ),
    span: Span {
        src (ptr): 0x00007fb12d4a1aa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
        ),
        start: 9,
        end: 36,
        as_str(): "const GLOBAL_VAL: u64 = 99;",
    },
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb12d4a1aa0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
            ),
            start: 41,
            end: 45,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    ConstantDeclaration(
                        DeclId(
                            545,
                            Span {
                                src (ptr): 0x00007fb12d4a1aa0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                ),
                                start: 67,
                                end: 76,
                                as_str(): "LOCAL_VAL",
                            },
                        ),
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb12d4a1aa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                    ),
                    start: 61,
                    end: 81,
                    as_str(): "const LOCAL_VAL = 1;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [
                                    BaseIdent {
                                        name_override_opt: Some(
                                            "core",
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12d4a1aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                            ),
                                            start: 97,
                                            end: 98,
                                            as_str(): "+",
                                        },
                                        is_raw_ident: false,
                                    },
                                    BaseIdent {
                                        name_override_opt: Some(
                                            "ops",
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12d4a1aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                            ),
                                            start: 97,
                                            end: 98,
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
                                        src (ptr): 0x00007fb12d4a1aa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                        ),
                                        start: 97,
                                        end: 98,
                                        as_str(): "+",
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
                                            src (ptr): 0x00007fb126c5c9c0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 124,
                                            end: 128,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                    ),
                                                    start: 15,
                                                    end: 25,
                                                    as_str(): "GLOBAL_VAL",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb12d4a1aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                ),
                                                start: 86,
                                                end: 96,
                                                as_str(): "GLOBAL_VAL",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12d4a1aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                            ),
                                            start: 86,
                                            end: 96,
                                            as_str(): "GLOBAL_VAL",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb126c5c9c0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 130,
                                            end: 135,
                                            as_str(): "other",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12d4a1aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                    ),
                                                    start: 67,
                                                    end: 76,
                                                    as_str(): "LOCAL_VAL",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb12d4a1aa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                                ),
                                                start: 99,
                                                end: 108,
                                                as_str(): "LOCAL_VAL",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12d4a1aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                            ),
                                            start: 99,
                                            end: 108,
                                            as_str(): "LOCAL_VAL",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                546,
                                Span {
                                    src (ptr): 0x00007fb126c5c9c0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                    ),
                                    start: 117,
                                    end: 185,
                                    as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb12d4a1aa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                                        ),
                                        start: 97,
                                        end: 98,
                                        as_str(): "+",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12d4a1aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                            ),
                            start: 86,
                            end: 108,
                            as_str(): "GLOBAL_VAL + LOCAL_VAL",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb12d4a1aa0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
                    ),
                    start: 86,
                    end: 108,
                    as_str(): "GLOBAL_VAL + LOCAL_VAL",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12d4a1aa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
        ),
        start: 38,
        end: 110,
        as_str(): "fn main() -> u64 {\n    const LOCAL_VAL = 1;\n    GLOBAL_VAL + LOCAL_VAL\n}",
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
        src (ptr): 0x00007fb12d4a1aa0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRzfkrHY/const_decl/src/main.sw",
        ),
        start: 51,
        end: 54,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

