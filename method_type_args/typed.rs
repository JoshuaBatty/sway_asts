TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0800b2770,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
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
        src (ptr): 0x00007fe0800b2770,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
        ),
        start: 9,
        end: 20,
        as_str(): "struct A {}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13316,
        Span {
            src (ptr): 0x00007fe0800b2770,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
            ),
            start: 22,
            end: 73,
            as_str(): "impl A {\n    fn generic<T>(self, x: T) -> T { x }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0800b2770,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
            ),
            start: 78,
            end: 81,
            as_str(): "foo",
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
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 110,
                                        as_str(): "generic",
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
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 49,
                                            end: 53,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: StructExpression {
                                            struct_name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0800b2770,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                    ),
                                                    start: 16,
                                                    end: 17,
                                                    as_str(): "A",
                                                },
                                                is_raw_ident: false,
                                            },
                                            fields: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0800b2770,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                ),
                                                start: 98,
                                                end: 99,
                                                as_str(): "A",
                                            },
                                        },
                                        return_type: TypeId(
                                            31632,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 102,
                                            as_str(): "A {}",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 55,
                                            end: 56,
                                            as_str(): "x",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 119,
                                            end: 123,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007fe0800b2770,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                    ),
                                    start: 35,
                                    end: 71,
                                    as_str(): "fn generic<T>(self, x: T) -> T { x }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [
                                        TypeArgument {
                                            type_id: TypeId(
                                                71,
                                            ),
                                            initial_type_id: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0800b2770,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                ),
                                                start: 113,
                                                end: 117,
                                                as_str(): "bool",
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 117,
                                        as_str(): "generic::<bool",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0800b2770,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                            ),
                            start: 98,
                            end: 124,
                            as_str(): "A {}.generic::<bool>(true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0800b2770,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                    ),
                    start: 98,
                    end: 124,
                    as_str(): "A {}.generic::<bool>(true)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0800b2770,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
        ),
        start: 75,
        end: 126,
        as_str(): "fn foo() -> bool {\n    A {}.generic::<bool>(true)\n}",
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
        src (ptr): 0x00007fe0800b2770,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
        ),
        start: 87,
        end: 91,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0800b2770,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
            ),
            start: 131,
            end: 135,
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
                                name_override_opt: Some(
                                    "_",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc01dd50,
                                    path: None,
                                    start: 0,
                                    end: 0,
                                    as_str(): "",
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
                                                src (ptr): 0x00007fe0800b2770,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                ),
                                                start: 152,
                                                end: 155,
                                                as_str(): "foo",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        13320,
                                        Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 75,
                                            end: 126,
                                            as_str(): "fn foo() -> bool {\n    A {}.generic::<bool>(true)\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0800b2770,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                ),
                                                start: 152,
                                                end: 155,
                                                as_str(): "foo",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0800b2770,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                    ),
                                    start: 152,
                                    end: 157,
                                    as_str(): "foo()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31668,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0800b2770,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                    ),
                    start: 144,
                    end: 158,
                    as_str(): "let _ = foo();",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0800b2770,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
        ),
        start: 128,
        end: 160,
        as_str(): "fn main() {\n    let _ = foo();\n}",
    },
    attributes: {},
    return_type: TypeId(
        31667,
    ),
    initial_return_type: TypeId(
        31666,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0800b2770,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
        ),
        start: 128,
        end: 137,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

