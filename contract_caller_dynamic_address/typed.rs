TyAbiDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb110b1a180,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
            ),
            start: 235,
            end: 242,
            as_str(): "SomeAbi",
        },
        is_raw_ident: false,
    },
    interface_surface: [
        DeclId(
            0,
            Span {
                src (ptr): 0x00007fb110b1a180,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                ),
                start: 250,
                end: 253,
                as_str(): "baz",
            },
        ),
        DeclId(
            1,
            Span {
                src (ptr): 0x00007fb110b1a180,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                ),
                start: 269,
                end: 273,
                as_str(): "quux",
            },
        ),
    ],
    methods: [],
    span: Span {
        src (ptr): 0x00007fb110b1a180,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
        ),
        start: 231,
        end: 285,
        as_str(): "abi SomeAbi {\n  fn baz() -> u32;\n  fn quux() -> u64;\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb110b1a180,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
            ),
            start: 14,
            end: 18,
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
                                    src (ptr): 0x00007fb110b1a180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                    ),
                                    start: 46,
                                    end: 52,
                                    as_str(): "caller",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: AbiCast {
                                    abi_name: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb110b1a180,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                ),
                                                start: 84,
                                                end: 91,
                                                as_str(): "SomeAbi",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    address: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb110b1a180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 23,
                                                    as_str(): "addr",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb110b1a180,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                ),
                                                start: 93,
                                                end: 97,
                                                as_str(): "addr",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            5,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb110b1a180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                            ),
                                            start: 93,
                                            end: 97,
                                            as_str(): "addr",
                                        },
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb110b1a180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                        ),
                                        start: 231,
                                        end: 285,
                                        as_str(): "abi SomeAbi {\n  fn baz() -> u32;\n  fn quux() -> u64;\n}",
                                    },
                                },
                                return_type: TypeId(
                                    7,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110b1a180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                    ),
                                    start: 231,
                                    end: 285,
                                    as_str(): "abi SomeAbi {\n  fn baz() -> u32;\n  fn quux() -> u64;\n}",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7,
                            ),
                            type_ascription: TypeId(
                                6,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb110b1a180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                    ),
                                    start: 54,
                                    end: 77,
                                    as_str(): "ContractCaller<SomeAbi>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb110b1a180,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                    ),
                    start: 42,
                    end: 99,
                    as_str(): "let caller: ContractCaller<SomeAbi> = abi(SomeAbi, addr);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: Some(
                                    "_",
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c011bb0,
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
                                                src (ptr): 0x00007fb110b1a180,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                ),
                                                start: 208,
                                                end: 211,
                                                as_str(): "baz",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        5,
                                        Span {
                                            src (ptr): 0x00007fb110b1a180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                            ),
                                            start: 250,
                                            end: 253,
                                            as_str(): "baz",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: Some(
                                        ContractCallParams {
                                            func_selector: [
                                                59,
                                                201,
                                                75,
                                                4,
                                            ],
                                            contract_address: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 19,
                                                            end: 23,
                                                            as_str(): "addr",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb110b1a180,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                        ),
                                                        start: 93,
                                                        end: 97,
                                                        as_str(): "addr",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    5,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb110b1a180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                    ),
                                                    start: 93,
                                                    end: 97,
                                                    as_str(): "addr",
                                                },
                                            },
                                        },
                                    ),
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb110b1a180,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                ),
                                                start: 208,
                                                end: 211,
                                                as_str(): "baz",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    2,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110b1a180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                    ),
                                    start: 201,
                                    end: 213,
                                    as_str(): "caller.baz()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                9,
                            ),
                            type_ascription: TypeId(
                                9,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb110b1a180,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                    ),
                    start: 193,
                    end: 214,
                    as_str(): "let _ = caller.baz();",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: Literal(
                                    U64(
                                        42,
                                    ),
                                ),
                                return_type: TypeId(
                                    3,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110b1a180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                    ),
                                    start: 224,
                                    end: 226,
                                    as_str(): "42",
                                },
                            },
                        ),
                        return_type: TypeId(
                            14,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb110b1a180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                            ),
                            start: 217,
                            end: 226,
                            as_str(): "return 42",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb110b1a180,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                    ),
                    start: 217,
                    end: 226,
                    as_str(): "return 42",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb110b1a180,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                    ),
                    start: 19,
                    end: 23,
                    as_str(): "addr",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                5,
            ),
            initial_type_id: TypeId(
                5,
            ),
            type_span: Span {
                src (ptr): 0x00007fb110b1a180,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                ),
                start: 25,
                end: 29,
                as_str(): "b256",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb110b1a180,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
        ),
        start: 11,
        end: 229,
        as_str(): "fn main(addr: b256) -> u64 {\n  let caller: ContractCaller<SomeAbi> = abi(SomeAbi, addr);\n  // this should revert since we don't have the script data being passed in to the harness\n  let _ = caller.baz();\n  return 42;\n}",
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
        src (ptr): 0x00007fb110b1a180,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
        ),
        start: 34,
        end: 37,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

