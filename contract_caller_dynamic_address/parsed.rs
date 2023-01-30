[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: ContractCaller {
                                                abi_name: Known(
                                                    CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 69,
                                                                end: 76,
                                                                as_str(): "SomeAbi",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                ),
                                                address: None,
                                            },
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
                                            body: Expression {
                                                kind: AbiCast(
                                                    AbiCastExpression {
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
                                                        address: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110b1a180,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                        ),
                                                                        start: 93,
                                                                        end: 97,
                                                                        as_str(): "addr",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
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
                                                span: Span {
                                                    src (ptr): 0x00007fb110b1a180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                    ),
                                                    start: 80,
                                                    end: 98,
                                                    as_str(): "abi(SomeAbi, addr)",
                                                },
                                            },
                                            is_mutable: false,
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
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
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
                                                            },
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
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110b1a180,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                            ),
                                                                            start: 201,
                                                                            end: 207,
                                                                            as_str(): "caller",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb110b1a180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                    ),
                                                                    start: 201,
                                                                    end: 207,
                                                                    as_str(): "caller",
                                                                },
                                                            },
                                                        ],
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        42,
                                                    ),
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fb110b1a180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                            ),
                            start: 38,
                            end: 229,
                            as_str(): "{\n  let caller: ContractCaller<SomeAbi> = abi(SomeAbi, addr);\n  // this should revert since we don't have the script data being passed in to the harness\n  let _ = caller.baz();\n  return 42;\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
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
                            type_info: B256,
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
                    span: Span {
                        src (ptr): 0x00007fb110b1a180,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                        ),
                        start: 11,
                        end: 229,
                        as_str(): "fn main(addr: b256) -> u64 {\n  let caller: ContractCaller<SomeAbi> = abi(SomeAbi, addr);\n  // this should revert since we don't have the script data being passed in to the harness\n  let _ = caller.baz();\n  return 42;\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb110b1a180,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
            ),
            start: 11,
            end: 229,
            as_str(): "fn main(addr: b256) -> u64 {\n  let caller: ContractCaller<SomeAbi> = abi(SomeAbi, addr);\n  // this should revert since we don't have the script data being passed in to the harness\n  let _ = caller.baz();\n  return 42;\n}",
        },
    },
    AstNode {
        content: Declaration(
            AbiDeclaration(
                AbiDeclaration {
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
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb110b1a180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                    ),
                                    start: 250,
                                    end: 253,
                                    as_str(): "baz",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [],
                            return_type: UnsignedInteger(
                                ThirtyTwo,
                            ),
                            return_type_span: Span {
                                src (ptr): 0x00007fb110b1a180,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                ),
                                start: 259,
                                end: 262,
                                as_str(): "u32",
                            },
                        },
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb110b1a180,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                    ),
                                    start: 269,
                                    end: 273,
                                    as_str(): "quux",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [],
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            return_type_span: Span {
                                src (ptr): 0x00007fb110b1a180,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                ),
                                start: 279,
                                end: 282,
                                as_str(): "u64",
                            },
                        },
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
                },
            ),
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
]
