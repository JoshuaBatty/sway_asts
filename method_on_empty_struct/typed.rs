TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0934e5bc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
            ),
            start: 16,
            end: 17,
            as_str(): "A",
        },
        is_raw_ident: false,
    },
    fields: [],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0934e5bc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
        ),
        start: 9,
        end: 21,
        as_str(): "struct A { }",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        546,
        Span {
            src (ptr): 0x00007fe0934e5bc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
            ),
            start: 23,
            end: 73,
            as_str(): "impl A {\n    fn f(self) -> u64 {\n        1\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0934e5bc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
            ),
            start: 78,
            end: 82,
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
                                    src (ptr): 0x00007fe0934e5bc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                    ),
                                    start: 102,
                                    end: 103,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 17,
                                            as_str(): "A",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 106,
                                        end: 107,
                                        as_str(): "A",
                                    },
                                },
                                return_type: TypeId(
                                    7252,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0934e5bc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                    ),
                                    start: 106,
                                    end: 111,
                                    as_str(): "A { }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7252,
                            ),
                            type_ascription: TypeId(
                                7268,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0934e5bc0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                    ),
                    start: 98,
                    end: 112,
                    as_str(): "let a = A { };",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 119,
                                        end: 120,
                                        as_str(): "f",
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
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 41,
                                            end: 45,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0934e5bc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 103,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0934e5bc0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                ),
                                                start: 117,
                                                end: 118,
                                                as_str(): "a",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7252,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 118,
                                            as_str(): "a",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                547,
                                Span {
                                    src (ptr): 0x00007fe0934e5bc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 71,
                                    as_str(): "fn f(self) -> u64 {\n        1\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 119,
                                        end: 120,
                                        as_str(): "f",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0934e5bc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                            ),
                            start: 117,
                            end: 122,
                            as_str(): "a.f()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0934e5bc0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                    ),
                    start: 117,
                    end: 122,
                    as_str(): "a.f()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0934e5bc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
        ),
        start: 75,
        end: 124,
        as_str(): "fn main() -> u64 {\n    let a = A { };\n    a.f()\n}",
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
        src (ptr): 0x00007fe0934e5bc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
        ),
        start: 88,
        end: 91,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

