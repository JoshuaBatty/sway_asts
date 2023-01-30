TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 16,
            end: 24,
            as_str(): "MyStruct",
        },
        is_raw_ident: false,
    },
    fields: [],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe08f6888f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
        ),
        start: 9,
        end: 28,
        as_str(): "struct MyStruct {\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 120,
            end: 123,
            as_str(): "fun",
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
                            src (ptr): 0x00007fe08f6888f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                            ),
                            start: 139,
                            end: 141,
                            as_str(): "42",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe08f6888f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                    ),
                    start: 139,
                    end: 141,
                    as_str(): "42",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe08f6888f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
        ),
        start: 117,
        end: 143,
        as_str(): "fn fun() -> u64 {\n    42\n}",
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
        src (ptr): 0x00007fe08f6888f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
        ),
        start: 129,
        end: 132,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
ImplTrait(
    DeclId(
        5,
        Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 30,
            end: 96,
            as_str(): "impl MyStruct {\n    pub fn my_fun() -> u64 {\n        fun()\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        6,
        Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 98,
            end: 115,
            as_str(): "impl MyStruct {\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe08f6888f0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
            ),
            start: 148,
            end: 152,
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
                                prefixes: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 168,
                                            end: 176,
                                            as_str(): "MyStruct",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 178,
                                        end: 184,
                                        as_str(): "my_fun",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                7,
                                Span {
                                    src (ptr): 0x00007fe08f6888f0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                    ),
                                    start: 50,
                                    end: 94,
                                    as_str(): "pub fn my_fun() -> u64 {\n        fun()\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 168,
                                        end: 184,
                                        as_str(): "MyStruct::my_fun",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            3,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe08f6888f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                            ),
                            start: 168,
                            end: 186,
                            as_str(): "MyStruct::my_fun()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe08f6888f0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                    ),
                    start: 168,
                    end: 186,
                    as_str(): "MyStruct::my_fun()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe08f6888f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
        ),
        start: 145,
        end: 188,
        as_str(): "fn main() -> u64 {\n    MyStruct::my_fun()\n}",
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
        src (ptr): 0x00007fe08f6888f0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
        ),
        start: 158,
        end: 161,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

