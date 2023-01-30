[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb10915a7d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                            ),
                            start: 14,
                            end: 17,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb10915a7d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                            ),
                            start: 19,
                            end: 30,
                            as_str(): "contract_id",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb10915a7d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                            ),
                            start: 32,
                            end: 42,
                            as_str(): "ContractId",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb10915a7d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
            ),
            start: 10,
            end: 43,
            as_str(): "use std::contract_id::ContractId;",
        },
    },
    AstNode {
        content: Declaration(
            AbiDeclaration(
                AbiDeclaration {
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
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb10915a7d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                    ),
                                    start: 69,
                                    end: 82,
                                    as_str(): "test_function",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [],
                            return_type: Boolean,
                            return_type_span: Span {
                                src (ptr): 0x00007fb10915a7d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                ),
                                start: 88,
                                end: 92,
                                as_str(): "bool",
                            },
                        },
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
                },
            ),
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
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb10915a7d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                ),
                                start: 102,
                                end: 112,
                                as_str(): "MyContract",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Contract,
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fb10915a7d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                        ),
                        start: 117,
                        end: 125,
                        as_str(): "Contract",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb10915a7d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 148,
                                    as_str(): "test_function",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb10915a7d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 173,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb10915a7d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                            ),
                                            start: 169,
                                            end: 173,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fb10915a7d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                    ),
                                    start: 159,
                                    end: 179,
                                    as_str(): "{\n        true\n    }",
                                },
                            },
                            parameters: [],
                            span: Span {
                                src (ptr): 0x00007fb10915a7d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                ),
                                start: 132,
                                end: 179,
                                as_str(): "fn test_function() -> bool {\n        true\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fb10915a7d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                ),
                                start: 154,
                                end: 158,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fb10915a7d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                        ),
                        start: 97,
                        end: 181,
                        as_str(): "impl MyContract for Contract {\n    fn test_function() -> bool {\n        true\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb10915a7d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
            ),
            start: 97,
            end: 181,
            as_str(): "impl MyContract for Contract {\n    fn test_function() -> bool {\n        true\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: AbiCast(
                                            AbiCastExpression {
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
                                                address: Expression {
                                                    kind: Subfield(
                                                        SubfieldExpression {
                                                            prefix: Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10915a7d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                            ),
                                                                            start: 255,
                                                                            end: 262,
                                                                            as_str(): "address",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
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
                                                            field_to_access: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10915a7d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                    ),
                                                                    start: 263,
                                                                    end: 268,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fb10915a7d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                            ),
                            start: 235,
                            end: 271,
                            as_str(): "{\n  abi(MyContract, address.value)\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
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
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 202,
                                        end: 212,
                                        as_str(): "ContractId",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
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
                    span: Span {
                        src (ptr): 0x00007fb10915a7d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                        ),
                        start: 183,
                        end: 271,
                        as_str(): "fn caller(address: ContractId) -> ContractCaller<_> {\n  abi(MyContract, address.value)\n}",
                    },
                    return_type: ContractCaller {
                        abi_name: Deferred,
                        address: None,
                    },
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb10915a7d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
            ),
            start: 183,
            end: 271,
            as_str(): "fn caller(address: ContractId) -> ContractCaller<_> {\n  abi(MyContract, address.value)\n}",
        },
    },
]
