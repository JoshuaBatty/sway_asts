TyAbiDeclaration {
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
        DeclId(
            544,
            Span {
                src (ptr): 0x00007f8a11564cc0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                ),
                start: 65,
                end: 76,
                as_str(): "impure_func",
            },
        ),
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
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                100,
                            ),
                        ),
                        return_type: TypeId(
                            7252,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a11564cc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
        ),
        start: 265,
        end: 302,
        as_str(): "fn can_read_only() -> u64 {\n    100\n}",
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
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
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
    visibility: Private,
    is_contract_call: false,
    purity: Reads,
}
TyFunctionDeclaration {
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
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                548,
                                Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 265,
                                    end: 302,
                                    as_str(): "fn can_read_only() -> u64 {\n    100\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            21,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a11564cc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
        ),
        start: 321,
        end: 375,
        as_str(): "fn can_also_read_only() -> u64 {\n    can_read_only()\n}",
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
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
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
    visibility: Private,
    is_contract_call: false,
    purity: Reads,
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                101,
                            ),
                        ),
                        return_type: TypeId(
                            7255,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a11564cc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
        ),
        start: 395,
        end: 433,
        as_str(): "fn can_write_only() -> u64 {\n    101\n}",
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
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
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
    visibility: Private,
    is_contract_call: false,
    purity: Writes,
}
TyFunctionDeclaration {
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
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                552,
                                Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 395,
                                    end: 433,
                                    as_str(): "fn can_write_only() -> u64 {\n    101\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            21,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a11564cc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
        ),
        start: 453,
        end: 509,
        as_str(): "fn can_also_write_only() -> u64 {\n    can_write_only()\n}",
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
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
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
    visibility: Private,
    is_contract_call: false,
    purity: Writes,
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
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
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        555,
                                        Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 321,
                                            end: 375,
                                            as_str(): "fn can_also_read_only() -> u64 {\n    can_read_only()\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
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
                                    ),
                                },
                                return_type: TypeId(
                                    21,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7258,
                            ),
                            type_ascription: TypeId(
                                7258,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
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
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        557,
                                        Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 453,
                                            end: 509,
                                            as_str(): "fn can_also_write_only() -> u64 {\n    can_write_only()\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
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
                                    ),
                                },
                                return_type: TypeId(
                                    21,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                7259,
                            ),
                            type_ascription: TypeId(
                                7259,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                102,
                            ),
                        ),
                        return_type: TypeId(
                            7260,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a11564cc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
        ),
        start: 535,
        end: 646,
        as_str(): "fn can_read_and_write() -> u64 {\n    let a = can_also_read_only();\n    let b = can_also_write_only();\n    102\n}",
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
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
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
    visibility: Private,
    is_contract_call: false,
    purity: ReadsWrites,
}
TyFunctionDeclaration {
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
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                560,
                                Span {
                                    src (ptr): 0x00007f8a11564cc0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                    ),
                                    start: 535,
                                    end: 646,
                                    as_str(): "fn can_read_and_write() -> u64 {\n    let a = can_also_read_only();\n    let b = can_also_write_only();\n    102\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            21,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a11564cc0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
        ),
        start: 683,
        end: 747,
        as_str(): "fn can_also_read_and_write() -> u64 {\n    can_read_and_write()\n}",
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
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
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
    visibility: Private,
    is_contract_call: false,
    purity: ReadsWrites,
}
ImplTrait(
    DeclId(
        565,
        Span {
            src (ptr): 0x00007f8a11564cc0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
            ),
            start: 91,
            end: 246,
            as_str(): "impl ImpurityTest for Contract {\n    #[storage(read, write)]\n    fn impure_func() -> bool {\n        let a = can_also_read_and_write();\n        true\n    }\n}",
        },
    ),
)

