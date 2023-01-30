




TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 91,
            end: 98,
            as_str(): "eq_test",
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
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 110,
                                    end: 112,
                                    as_str(): "w1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 115,
                                                    end: 122,
                                                    as_str(): "Wrapper",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0dac0ad90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                ),
                                                start: 124,
                                                end: 127,
                                                as_str(): "new",
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
                                                    src (ptr): 0x00007fe0d90fed50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                    ),
                                                    start: 127,
                                                    end: 132,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        3,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 128,
                                                    end: 129,
                                                    as_str(): "3",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13343,
                                        Span {
                                            src (ptr): 0x00007fe0d90fed50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                            ),
                                            start: 116,
                                            end: 245,
                                            as_str(): "pub fn new(value: u64) -> Self {\n        Wrapper {\n            asset: Asset {\n                value\n            }\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0dac0ad90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                ),
                                                start: 115,
                                                end: 127,
                                                as_str(): "Wrapper::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31724,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 115,
                                    end: 130,
                                    as_str(): "Wrapper::new(3)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31724,
                            ),
                            type_ascription: TypeId(
                                31817,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 106,
                    end: 131,
                    as_str(): "let w1 = Wrapper::new(3);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 139,
                                    end: 141,
                                    as_str(): "w2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 144,
                                                    end: 151,
                                                    as_str(): "Wrapper",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0dac0ad90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                ),
                                                start: 153,
                                                end: 156,
                                                as_str(): "new",
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
                                                    src (ptr): 0x00007fe0d90fed50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                    ),
                                                    start: 127,
                                                    end: 132,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        3,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 158,
                                                    as_str(): "3",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13344,
                                        Span {
                                            src (ptr): 0x00007fe0d90fed50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                            ),
                                            start: 116,
                                            end: 245,
                                            as_str(): "pub fn new(value: u64) -> Self {\n        Wrapper {\n            asset: Asset {\n                value\n            }\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0dac0ad90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                ),
                                                start: 144,
                                                end: 156,
                                                as_str(): "Wrapper::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31724,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 144,
                                    end: 159,
                                    as_str(): "Wrapper::new(3)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31724,
                            ),
                            type_ascription: TypeId(
                                31821,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 135,
                    end: 160,
                    as_str(): "let w2 = Wrapper::new(3);",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 165,
                                        end: 171,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe0e2cb2f40,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 177,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 177,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 175,
                                                        end: 177,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d90fed50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                            ),
                                                            start: 281,
                                                            end: 285,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                    ),
                                                                    start: 110,
                                                                    end: 112,
                                                                    as_str(): "w1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 172,
                                                                end: 174,
                                                                as_str(): "w1",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31724,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 174,
                                                            as_str(): "w1",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d90fed50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                            ),
                                                            start: 287,
                                                            end: 292,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                    ),
                                                                    start: 139,
                                                                    end: 141,
                                                                    as_str(): "w2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 178,
                                                                end: 180,
                                                                as_str(): "w2",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31724,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 178,
                                                            end: 180,
                                                            as_str(): "w2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13346,
                                                Span {
                                                    src (ptr): 0x00007fe0d90fed50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                    ),
                                                    start: 275,
                                                    end: 349,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        self.asset == other.asset\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 175,
                                                        end: 177,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 172,
                                            end: 180,
                                            as_str(): "w1 == w2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13347,
                                Span {
                                    src (ptr): 0x00007fe0e2cb2f40,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 165,
                                        end: 171,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31829,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 165,
                            end: 181,
                            as_str(): "assert(w1 == w2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 165,
                    end: 181,
                    as_str(): "assert(w1 == w2)",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 186,
                                        end: 192,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe0e2cb2f40,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 202,
                                                            end: 204,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 202,
                                                            end: 204,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 202,
                                                        end: 204,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dabeaf30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                            ),
                                                            start: 103,
                                                            end: 107,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 110,
                                                                            end: 112,
                                                                            as_str(): "w1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                        ),
                                                                        start: 193,
                                                                        end: 195,
                                                                        as_str(): "w1",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31724,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                    ),
                                                                    start: 193,
                                                                    end: 195,
                                                                    as_str(): "w1",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                        ),
                                                                        start: 81,
                                                                        end: 86,
                                                                        as_str(): "asset",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    31665,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    31722,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 93,
                                                                    as_str(): "asset: Asset",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                    ),
                                                                    start: 88,
                                                                    end: 93,
                                                                    as_str(): "Asset",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 201,
                                                                as_str(): "asset",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31724,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31665,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 193,
                                                            end: 201,
                                                            as_str(): "w1.asset",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dabeaf30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                            ),
                                                            start: 109,
                                                            end: 114,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 139,
                                                                            end: 141,
                                                                            as_str(): "w2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                        ),
                                                                        start: 205,
                                                                        end: 207,
                                                                        as_str(): "w2",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31724,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                    ),
                                                                    start: 205,
                                                                    end: 207,
                                                                    as_str(): "w2",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d90fed50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                        ),
                                                                        start: 81,
                                                                        end: 86,
                                                                        as_str(): "asset",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    31665,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    31722,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 93,
                                                                    as_str(): "asset: Asset",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0d90fed50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/utils.sw",
                                                                    ),
                                                                    start: 88,
                                                                    end: 93,
                                                                    as_str(): "Asset",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 208,
                                                                end: 213,
                                                                as_str(): "asset",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31724,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31665,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 205,
                                                            end: 213,
                                                            as_str(): "w2.asset",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13349,
                                                Span {
                                                    src (ptr): 0x00007fe0dabeaf30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/asset.sw",
                                                    ),
                                                    start: 97,
                                                    end: 171,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        self.value == other.value\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 202,
                                                        end: 204,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 193,
                                            end: 213,
                                            as_str(): "w1.asset == w2.asset",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13350,
                                Span {
                                    src (ptr): 0x00007fe0e2cb2f40,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 186,
                                        end: 192,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31836,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 186,
                            end: 214,
                            as_str(): "assert(w1.asset == w2.asset)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 186,
                    end: 214,
                    as_str(): "assert(w1.asset == w2.asset)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0dac0ad90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
        ),
        start: 88,
        end: 217,
        as_str(): "fn eq_test() {\n   let w1 = Wrapper::new(3);\n   let w2 = Wrapper::new(3);\n\n   assert(w1 == w2);\n   assert(w1.asset == w2.asset);\n}",
    },
    attributes: {},
    return_type: TypeId(
        31816,
    ),
    initial_return_type: TypeId(
        31815,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0dac0ad90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
        ),
        start: 88,
        end: 100,
        as_str(): "fn eq_test()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 222,
            end: 226,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 241,
                                        end: 248,
                                        as_str(): "eq_test",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13353,
                                Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 88,
                                    end: 217,
                                    as_str(): "fn eq_test() {\n   let w1 = Wrapper::new(3);\n   let w2 = Wrapper::new(3);\n\n   assert(w1 == w2);\n   assert(w1.asset == w2.asset);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 241,
                                        end: 248,
                                        as_str(): "eq_test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31840,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 241,
                            end: 250,
                            as_str(): "eq_test()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 241,
                    end: 250,
                    as_str(): "eq_test()",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 260,
                                    end: 261,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 264,
                                                    end: 271,
                                                    as_str(): "Context",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0dac0ad90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                ),
                                                start: 273,
                                                end: 276,
                                                as_str(): "foo",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        13354,
                                        Span {
                                            src (ptr): 0x00007fe0db5dcf00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                            ),
                                            start: 75,
                                            end: 131,
                                            as_str(): "pub fn foo() -> Self {\n    Context { something: 10 }\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0dac0ad90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                ),
                                                start: 264,
                                                end: 276,
                                                as_str(): "Context::foo",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31633,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 264,
                                    end: 278,
                                    as_str(): "Context::foo()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31633,
                            ),
                            type_ascription: TypeId(
                                31841,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 256,
                    end: 279,
                    as_str(): "let x = Context::foo();",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructFieldAccess {
                            prefix: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 260,
                                            end: 261,
                                            as_str(): "x",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0dac0ad90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                        ),
                                        start: 283,
                                        end: 284,
                                        as_str(): "x",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    31633,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 283,
                                    end: 284,
                                    as_str(): "x",
                                },
                            },
                            field_to_access: TyStructField {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0db5dcf00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                        ),
                                        start: 40,
                                        end: 49,
                                        as_str(): "something",
                                    },
                                    is_raw_ident: false,
                                },
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0db5dcf00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                    ),
                                    start: 40,
                                    end: 54,
                                    as_str(): "something: u64",
                                },
                                type_span: Span {
                                    src (ptr): 0x00007fe0db5dcf00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/context.sw",
                                    ),
                                    start: 51,
                                    end: 54,
                                    as_str(): "u64",
                                },
                                attributes: {},
                            },
                            field_instantiation_span: Span {
                                src (ptr): 0x00007fe0dac0ad90,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                ),
                                start: 285,
                                end: 294,
                                as_str(): "something",
                            },
                            resolved_type_of_parent: TypeId(
                                31633,
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 283,
                            end: 294,
                            as_str(): "x.something",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 283,
                    end: 294,
                    as_str(): "x.something",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0dac0ad90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
        ),
        start: 219,
        end: 296,
        as_str(): "fn main() -> u64 {\n   eq_test();\n\n   let x = Context::foo();\n   x.something\n}",
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
        src (ptr): 0x00007fe0dac0ad90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
        ),
        start: 232,
        end: 235,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

