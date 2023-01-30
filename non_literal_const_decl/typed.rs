TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe07c9e90e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
            ),
            start: 51,
            end: 59,
            as_str(): "a_number",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                42,
                            ),
                        ),
                        return_type: TypeId(
                            4,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 100,
                            end: 102,
                            as_str(): "42",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe07c9e90e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                    ),
                    start: 100,
                    end: 102,
                    as_str(): "42",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe07c9e90e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                    ),
                    start: 60,
                    end: 62,
                    as_str(): "_a",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                3,
            ),
            type_span: Span {
                src (ptr): 0x00007fe07c9e90e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                ),
                start: 64,
                end: 67,
                as_str(): "u64",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe07c9e90e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                    ),
                    start: 69,
                    end: 71,
                    as_str(): "_b",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                3,
            ),
            type_span: Span {
                src (ptr): 0x00007fe07c9e90e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                ),
                start: 73,
                end: 76,
                as_str(): "u64",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe07c9e90e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                    ),
                    start: 78,
                    end: 80,
                    as_str(): "_c",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                3,
            ),
            initial_type_id: TypeId(
                3,
            ),
            type_span: Span {
                src (ptr): 0x00007fe07c9e90e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                ),
                start: 82,
                end: 85,
                as_str(): "u64",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe07c9e90e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
        ),
        start: 48,
        end: 104,
        as_str(): "fn a_number(_a: u64, _b: u64, _c: u64) -> u64 {\n    42\n}",
    },
    attributes: {},
    return_type: TypeId(
        3,
    ),
    initial_return_type: TypeId(
        3,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe07c9e90e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
        ),
        start: 90,
        end: 93,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe07c9e90e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
            ),
            start: 15,
            end: 25,
            as_str(): "GLOBAL_NUM",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: FunctionApplication {
            call_path: CallPath {
                prefixes: [],
                suffix: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fe07c9e90e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                        ),
                        start: 28,
                        end: 36,
                        as_str(): "a_number",
                    },
                    is_raw_ident: false,
                },
                is_absolute: false,
            },
            contract_call_params: {},
            arguments: [
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 60,
                            end: 62,
                            as_str(): "_a",
                        },
                        is_raw_ident: false,
                    },
                    TyExpression {
                        expression: Literal(
                            U64(
                                1,
                            ),
                        ),
                        return_type: TypeId(
                            3,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 37,
                            end: 38,
                            as_str(): "1",
                        },
                    },
                ),
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 69,
                            end: 71,
                            as_str(): "_b",
                        },
                        is_raw_ident: false,
                    },
                    TyExpression {
                        expression: Literal(
                            U64(
                                2,
                            ),
                        ),
                        return_type: TypeId(
                            3,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 40,
                            end: 41,
                            as_str(): "2",
                        },
                    },
                ),
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 78,
                            end: 80,
                            as_str(): "_c",
                        },
                        is_raw_ident: false,
                    },
                    TyExpression {
                        expression: Literal(
                            U64(
                                3,
                            ),
                        ),
                        return_type: TypeId(
                            3,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 43,
                            end: 44,
                            as_str(): "3",
                        },
                    },
                ),
            ],
            function_decl_id: DeclId(
                2,
                Span {
                    src (ptr): 0x00007fe07c9e90e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                    ),
                    start: 48,
                    end: 104,
                    as_str(): "fn a_number(_a: u64, _b: u64, _c: u64) -> u64 {\n    42\n}",
                },
            ),
            self_state_idx: None,
            selector: None,
            type_binding: Some(
                TypeBinding {
                    inner: (),
                    type_arguments: [],
                    span: Span {
                        src (ptr): 0x00007fe07c9e90e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                        ),
                        start: 28,
                        end: 36,
                        as_str(): "a_number",
                    },
                },
            ),
        },
        return_type: TypeId(
            3,
        ),
        span: Span {
            src (ptr): 0x00007fe07c9e90e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
            ),
            start: 28,
            end: 45,
            as_str(): "a_number(1, 2, 3)",
        },
    },
    visibility: Private,
    return_type: TypeId(
        5,
    ),
    is_configurable: false,
    attributes: {},
    type_ascription_span: None,
    span: Span {
        src (ptr): 0x00007fe07c9e90e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
        ),
        start: 9,
        end: 46,
        as_str(): "const GLOBAL_NUM = a_number(1, 2, 3);",
    },
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe07c9e90e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
            ),
            start: 109,
            end: 113,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 133,
                                    end: 134,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe07c9e90e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                ),
                                                start: 137,
                                                end: 145,
                                                as_str(): "a_number",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 60,
                                                    end: 62,
                                                    as_str(): "_a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    3,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 146,
                                                    end: 147,
                                                    as_str(): "4",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 69,
                                                    end: 71,
                                                    as_str(): "_b",
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
                                                    3,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 149,
                                                    end: 150,
                                                    as_str(): "5",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 78,
                                                    end: 80,
                                                    as_str(): "_c",
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
                                                    3,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 152,
                                                    end: 153,
                                                    as_str(): "6",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        5,
                                        Span {
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 104,
                                            as_str(): "fn a_number(_a: u64, _b: u64, _c: u64) -> u64 {\n    42\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe07c9e90e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                ),
                                                start: 137,
                                                end: 145,
                                                as_str(): "a_number",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    3,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 137,
                                    end: 154,
                                    as_str(): "a_number(4, 5, 6)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                13,
                            ),
                            type_ascription: TypeId(
                                13,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe07c9e90e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                    ),
                    start: 129,
                    end: 155,
                    as_str(): "let a = a_number(4, 5, 6);",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c9e90e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                    ),
                                    start: 15,
                                    end: 25,
                                    as_str(): "GLOBAL_NUM",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe07c9e90e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                ),
                                start: 160,
                                end: 170,
                                as_str(): "GLOBAL_NUM",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            5,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 160,
                            end: 170,
                            as_str(): "GLOBAL_NUM",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe07c9e90e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                    ),
                    start: 160,
                    end: 170,
                    as_str(): "GLOBAL_NUM",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe07c9e90e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
        ),
        start: 106,
        end: 172,
        as_str(): "fn main() -> u64 {\n    let a = a_number(4, 5, 6);\n    GLOBAL_NUM\n}",
    },
    attributes: {},
    return_type: TypeId(
        3,
    ),
    initial_return_type: TypeId(
        3,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe07c9e90e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
        ),
        start: 119,
        end: 122,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

