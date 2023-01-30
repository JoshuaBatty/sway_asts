

TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb1359d0020,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
            ),
            start: 45,
            end: 49,
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
                                        src (ptr): 0x00007fb1359d0020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 70,
                                        as_str(): "adder",
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
                                    src (ptr): 0x00007fb12be71f40,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/consts.sw",
                                    ),
                                    start: 61,
                                    end: 103,
                                    as_str(): "pub fn adder() -> u64 {\n    THREE + FOUR\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb1359d0020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 70,
                                        as_str(): "adder",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1359d0020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                            ),
                            start: 65,
                            end: 72,
                            as_str(): "adder()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb1359d0020,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
                    ),
                    start: 65,
                    end: 72,
                    as_str(): "adder()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb1359d0020,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
        ),
        start: 42,
        end: 74,
        as_str(): "fn main() -> u64 {\n    adder()\n}",
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
        src (ptr): 0x00007fb1359d0020,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRLoOEEw/const_decl_and_use_in_library/src/main.sw",
        ),
        start: 55,
        end: 58,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

