TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c8c39d20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
            ),
            start: 12,
            end: 15,
            as_str(): "ten",
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
                                10,
                            ),
                        ),
                        return_type: TypeId(
                            7252,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 31,
                            end: 33,
                            as_str(): "10",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c8c39d20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                    ),
                    start: 31,
                    end: 33,
                    as_str(): "10",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c8c39d20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
        ),
        start: 9,
        end: 35,
        as_str(): "fn ten() -> u64 {\n    10\n}",
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
        src (ptr): 0x00007fe0c8c39d20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
        ),
        start: 21,
        end: 24,
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
            src (ptr): 0x00007fe0c8c39d20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
            ),
            start: 40,
            end: 43,
            as_str(): "nop",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0c8c39d20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                        ),
                                        start: 52,
                                        end: 55,
                                        as_str(): "ten",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                546,
                                Span {
                                    src (ptr): 0x00007fe0c8c39d20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                    ),
                                    start: 9,
                                    end: 35,
                                    as_str(): "fn ten() -> u64 {\n    10\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c8c39d20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                        ),
                                        start: 52,
                                        end: 55,
                                        as_str(): "ten",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 52,
                            end: 57,
                            as_str(): "ten()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c8c39d20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                    ),
                    start: 52,
                    end: 57,
                    as_str(): "ten()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c8c39d20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
        ),
        start: 37,
        end: 60,
        as_str(): "fn nop() {\n    ten();\n}",
    },
    attributes: {},
    return_type: TypeId(
        7255,
    ),
    initial_return_type: TypeId(
        7254,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0c8c39d20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
        ),
        start: 37,
        end: 45,
        as_str(): "fn nop()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c8c39d20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
            ),
            start: 65,
            end: 69,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0c8c39d20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                        ),
                                        start: 78,
                                        end: 81,
                                        as_str(): "nop",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                549,
                                Span {
                                    src (ptr): 0x00007fe0c8c39d20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                    ),
                                    start: 37,
                                    end: 60,
                                    as_str(): "fn nop() {\n    ten();\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c8c39d20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                        ),
                                        start: 78,
                                        end: 81,
                                        as_str(): "nop",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7261,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 78,
                            end: 83,
                            as_str(): "nop()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c8c39d20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                    ),
                    start: 78,
                    end: 83,
                    as_str(): "nop()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c8c39d20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
        ),
        start: 62,
        end: 85,
        as_str(): "fn main() {\n    nop()\n}",
    },
    attributes: {},
    return_type: TypeId(
        7260,
    ),
    initial_return_type: TypeId(
        7259,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0c8c39d20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
        ),
        start: 62,
        end: 71,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

