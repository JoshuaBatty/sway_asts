[
    AstNode {
        content: Declaration(
            AbiDeclaration(
                AbiDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 15,
                            end: 27,
                            as_str(): "ImpurityTest",
                        },
                        is_raw_ident: false,
                    },
                    interface_surface: [
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 76,
                                    as_str(): "impure_func",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {
                                Storage: [
                                    Attribute {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007f8a11564cc0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                ),
                                                start: 36,
                                                end: 43,
                                                as_str(): "storage",
                                            },
                                            is_raw_ident: false,
                                        },
                                        args: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 44,
                                                    end: 48,
                                                    as_str(): "read",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 50,
                                                    end: 55,
                                                    as_str(): "write",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 34,
                                            end: 57,
                                            as_str(): "#[storage(read, write)]",
                                        },
                                    },
                                ],
                            },
                            purity: ReadsWrites,
                            parameters: [],
                            return_type: Boolean,
                            return_type_span: Span {
                                src (ptr): 0x00007f8a11564cc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                ),
                                start: 82,
                                end: 86,
                                as_str(): "bool",
                            },
                        },
                    ],
                    methods: [],
                    span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 11,
                        end: 89,
                        as_str(): "abi ImpurityTest {\n    #[storage(read, write)]\n    fn impure_func() -> bool;\n}",
                    },
                    attributes: {},
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 11,
            end: 89,
            as_str(): "abi ImpurityTest {\n    #[storage(read, write)]\n    fn impure_func() -> bool;\n}",
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
                                src (ptr): 0x00007f8a11564cc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                ),
                                start: 96,
                                end: 108,
                                as_str(): "ImpurityTest",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Contract,
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 113,
                        end: 121,
                        as_str(): "Contract",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: ReadsWrites,
                            attributes: {
                                Storage: [
                                    Attribute {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007f8a11564cc0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                ),
                                                start: 130,
                                                end: 137,
                                                as_str(): "storage",
                                            },
                                            is_raw_ident: false,
                                        },
                                        args: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 138,
                                                    end: 142,
                                                    as_str(): "read",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 144,
                                                    end: 149,
                                                    as_str(): "write",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 151,
                                            as_str(): "#[storage(read, write)]",
                                        },
                                    },
                                ],
                            },
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 159,
                                    end: 170,
                                    as_str(): "impure_func",
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
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 195,
                                                            end: 196,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    type_ascription: Unknown,
                                                    type_ascription_span: None,
                                                    body: Expression {
                                                        kind: FunctionApplication(
                                                            FunctionApplicationExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                ),
                                                                                start: 199,
                                                                                end: 222,
                                                                                as_str(): "can_also_read_and_write",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 199,
                                                                        end: 222,
                                                                        as_str(): "can_also_read_and_write",
                                                                    },
                                                                },
                                                                arguments: [],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 224,
                                                            as_str(): "can_also_read_and_write()",
                                                        },
                                                    },
                                                    is_mutable: false,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 191,
                                            end: 225,
                                            as_str(): "let a = can_also_read_and_write();",
                                        },
                                    },
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 234,
                                                    end: 238,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 234,
                                            end: 238,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 181,
                                    end: 244,
                                    as_str(): "{\n        let a = can_also_read_and_write();\n        true\n    }",
                                },
                            },
                            parameters: [],
                            span: Span {
                                src (ptr): 0x00007f8a11564cc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                ),
                                start: 156,
                                end: 244,
                                as_str(): "fn impure_func() -> bool {\n        let a = can_also_read_and_write();\n        true\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007f8a11564cc0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                ),
                                start: 176,
                                end: 180,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 91,
                        end: 246,
                        as_str(): "impl ImpurityTest for Contract {\n    #[storage(read, write)]\n    fn impure_func() -> bool {\n        let a = can_also_read_and_write();\n        true\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 91,
            end: 246,
            as_str(): "impl ImpurityTest for Contract {\n    #[storage(read, write)]\n    fn impure_func() -> bool {\n        let a = can_also_read_and_write();\n        true\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Reads,
                    attributes: {
                        Storage: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 250,
                                        end: 257,
                                        as_str(): "storage",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 258,
                                            end: 262,
                                            as_str(): "read",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 248,
                                    end: 264,
                                    as_str(): "#[storage(read)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 268,
                            end: 281,
                            as_str(): "can_read_only",
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
                                            Numeric(
                                                100,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 297,
                                            end: 300,
                                            as_str(): "100",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 297,
                                    end: 300,
                                    as_str(): "100",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 291,
                            end: 302,
                            as_str(): "{\n    100\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 265,
                        end: 302,
                        as_str(): "fn can_read_only() -> u64 {\n    100\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 287,
                        end: 290,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 248,
            end: 302,
            as_str(): "#[storage(read)]\nfn can_read_only() -> u64 {\n    100\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Reads,
                    attributes: {
                        Storage: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 306,
                                        end: 313,
                                        as_str(): "storage",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 314,
                                            end: 318,
                                            as_str(): "read",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 304,
                                    end: 320,
                                    as_str(): "#[storage(read)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 324,
                            end: 342,
                            as_str(): "can_also_read_only",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 358,
                                                                end: 371,
                                                                as_str(): "can_read_only",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 358,
                                                        end: 371,
                                                        as_str(): "can_read_only",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 358,
                                            end: 373,
                                            as_str(): "can_read_only()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 358,
                                    end: 373,
                                    as_str(): "can_read_only()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 352,
                            end: 375,
                            as_str(): "{\n    can_read_only()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 321,
                        end: 375,
                        as_str(): "fn can_also_read_only() -> u64 {\n    can_read_only()\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 348,
                        end: 351,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 304,
            end: 375,
            as_str(): "#[storage(read)]\nfn can_also_read_only() -> u64 {\n    can_read_only()\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Writes,
                    attributes: {
                        Storage: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 379,
                                        end: 386,
                                        as_str(): "storage",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 387,
                                            end: 392,
                                            as_str(): "write",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 377,
                                    end: 394,
                                    as_str(): "#[storage(write)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 398,
                            end: 412,
                            as_str(): "can_write_only",
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
                                            Numeric(
                                                101,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 428,
                                            end: 431,
                                            as_str(): "101",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 428,
                                    end: 431,
                                    as_str(): "101",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 422,
                            end: 433,
                            as_str(): "{\n    101\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 395,
                        end: 433,
                        as_str(): "fn can_write_only() -> u64 {\n    101\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 418,
                        end: 421,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 377,
            end: 433,
            as_str(): "#[storage(write)]\nfn can_write_only() -> u64 {\n    101\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Writes,
                    attributes: {
                        Storage: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 437,
                                        end: 444,
                                        as_str(): "storage",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 445,
                                            end: 450,
                                            as_str(): "write",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 435,
                                    end: 452,
                                    as_str(): "#[storage(write)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 456,
                            end: 475,
                            as_str(): "can_also_write_only",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 491,
                                                                end: 505,
                                                                as_str(): "can_write_only",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 491,
                                                        end: 505,
                                                        as_str(): "can_write_only",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 491,
                                            end: 507,
                                            as_str(): "can_write_only()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 491,
                                    end: 507,
                                    as_str(): "can_write_only()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 485,
                            end: 509,
                            as_str(): "{\n    can_write_only()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 453,
                        end: 509,
                        as_str(): "fn can_also_write_only() -> u64 {\n    can_write_only()\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 481,
                        end: 484,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 435,
            end: 509,
            as_str(): "#[storage(write)]\nfn can_also_write_only() -> u64 {\n    can_write_only()\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: ReadsWrites,
                    attributes: {
                        Storage: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 513,
                                        end: 520,
                                        as_str(): "storage",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 521,
                                            end: 525,
                                            as_str(): "read",
                                        },
                                        is_raw_ident: false,
                                    },
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 527,
                                            end: 532,
                                            as_str(): "write",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 511,
                                    end: 534,
                                    as_str(): "#[storage(read, write)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 538,
                            end: 556,
                            as_str(): "can_read_and_write",
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
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 576,
                                                    end: 577,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 580,
                                                                        end: 598,
                                                                        as_str(): "can_also_read_only",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 580,
                                                                end: 598,
                                                                as_str(): "can_also_read_only",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 580,
                                                    end: 600,
                                                    as_str(): "can_also_read_only()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 572,
                                    end: 601,
                                    as_str(): "let a = can_also_read_only();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 610,
                                                    end: 611,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 614,
                                                                        end: 633,
                                                                        as_str(): "can_also_write_only",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 614,
                                                                end: 633,
                                                                as_str(): "can_also_write_only",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 614,
                                                    end: 635,
                                                    as_str(): "can_also_write_only()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 606,
                                    end: 636,
                                    as_str(): "let b = can_also_write_only();",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                102,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 641,
                                            end: 644,
                                            as_str(): "102",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 641,
                                    end: 644,
                                    as_str(): "102",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 566,
                            end: 646,
                            as_str(): "{\n    let a = can_also_read_only();\n    let b = can_also_write_only();\n    102\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 535,
                        end: 646,
                        as_str(): "fn can_read_and_write() -> u64 {\n    let a = can_also_read_only();\n    let b = can_also_write_only();\n    102\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 562,
                        end: 565,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 511,
            end: 646,
            as_str(): "#[storage(read, write)]\nfn can_read_and_write() -> u64 {\n    let a = can_also_read_only();\n    let b = can_also_write_only();\n    102\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: ReadsWrites,
                    attributes: {
                        Storage: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 650,
                                        end: 657,
                                        as_str(): "storage",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 658,
                                            end: 662,
                                            as_str(): "read",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 648,
                                    end: 664,
                                    as_str(): "#[storage(read)]",
                                },
                            },
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 667,
                                        end: 674,
                                        as_str(): "storage",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 675,
                                            end: 680,
                                            as_str(): "write",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 665,
                                    end: 682,
                                    as_str(): "#[storage(write)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 686,
                            end: 709,
                            as_str(): "can_also_read_and_write",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 725,
                                                                end: 743,
                                                                as_str(): "can_read_and_write",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 725,
                                                        end: 743,
                                                        as_str(): "can_read_and_write",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 725,
                                            end: 745,
                                            as_str(): "can_read_and_write()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 725,
                                    end: 745,
                                    as_str(): "can_read_and_write()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 719,
                            end: 747,
                            as_str(): "{\n    can_read_and_write()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 683,
                        end: 747,
                        as_str(): "fn can_also_read_and_write() -> u64 {\n    can_read_and_write()\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 715,
                        end: 718,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 648,
            end: 747,
            as_str(): "#[storage(read)]\n#[storage(write)]\nfn can_also_read_and_write() -> u64 {\n    can_read_and_write()\n}",
        },
    },
]
