
TyAbiDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb10915a7d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
            ),
            start: 49,
            end: 59,
            as_str(): "MyContract",
        },
        is_raw_ident: false,
    },
    interface_surface: [
        DeclId(
            13314,
            Span {
                src (ptr): 0x00007fb10915a7d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                ),
                start: 69,
                end: 82,
                as_str(): "test_function",
            },
        ),
    ],
    methods: [],
    span: Span {
        src (ptr): 0x00007fb10915a7d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
        ),
        start: 45,
        end: 95,
        as_str(): "abi MyContract {\n    fn test_function() -> bool;\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13317,
        Span {
            src (ptr): 0x00007fb10915a7d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
            ),
            start: 97,
            end: 181,
            as_str(): "impl MyContract for Contract {\n    fn test_function() -> bool {\n        true\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb10915a7d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
            ),
            start: 186,
            end: 192,
            as_str(): "caller",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: AbiCast {
                            abi_name: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 243,
                                        end: 253,
                                        as_str(): "MyContract",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            address: TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10915a7d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                    ),
                                                    start: 193,
                                                    end: 200,
                                                    as_str(): "address",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb10915a7d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                ),
                                                start: 255,
                                                end: 262,
                                                as_str(): "address",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7861,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb10915a7d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                            ),
                                            start: 255,
                                            end: 262,
                                            as_str(): "address",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb118716190,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                                ),
                                                start: 243,
                                                end: 248,
                                                as_str(): "value",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_id: TypeId(
                                            59,
                                        ),
                                        initial_type_id: TypeId(
                                            59,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb118716190,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                            ),
                                            start: 243,
                                            end: 254,
                                            as_str(): "value: b256",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fb118716190,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/contract_id.sw",
                                            ),
                                            start: 250,
                                            end: 254,
                                            as_str(): "b256",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 263,
                                        end: 268,
                                        as_str(): "value",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7861,
                                    ),
                                },
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb10915a7d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                    ),
                                    start: 255,
                                    end: 268,
                                    as_str(): "address.value",
                                },
                            },
                            span: Span {
                                src (ptr): 0x00007fb10915a7d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                ),
                                start: 45,
                                end: 95,
                                as_str(): "abi MyContract {\n    fn test_function() -> bool;\n}",
                            },
                        },
                        return_type: TypeId(
                            31683,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb10915a7d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                            ),
                            start: 45,
                            end: 95,
                            as_str(): "abi MyContract {\n    fn test_function() -> bool;\n}",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb10915a7d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                    ),
                    start: 239,
                    end: 269,
                    as_str(): "abi(MyContract, address.value)",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb10915a7d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                    ),
                    start: 193,
                    end: 200,
                    as_str(): "address",
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
                7861,
            ),
            initial_type_id: TypeId(
                31680,
            ),
            type_span: Span {
                src (ptr): 0x00007fb10915a7d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                ),
                start: 202,
                end: 212,
                as_str(): "ContractId",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb10915a7d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
        ),
        start: 183,
        end: 271,
        as_str(): "fn caller(address: ContractId) -> ContractCaller<_> {\n  abi(MyContract, address.value)\n}",
    },
    attributes: {},
    return_type: TypeId(
        31681,
    ),
    initial_return_type: TypeId(
        31681,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb10915a7d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
        ),
        start: 217,
        end: 234,
        as_str(): "ContractCaller<_>",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

