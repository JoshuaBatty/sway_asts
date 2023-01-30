


TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0f57bed10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
            ),
            start: 80,
            end: 84,
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
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 112,
                                    as_str(): "hi_bits",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                            119,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 121,
                                    end: 187,
                                    as_str(): "0x7777777777777777777777777777777777777777777777777777777777777777",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                59,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 118,
                                    as_str(): "b256",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f57bed10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                    ),
                    start: 101,
                    end: 188,
                    as_str(): "let hi_bits: b256 = 0x7777777777777777777777777777777777777777777777777777777777777777;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 197,
                                    end: 204,
                                    as_str(): "lo_bits",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                            0,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 213,
                                    end: 279,
                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                59,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 206,
                                    end: 210,
                                    as_str(): "b256",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f57bed10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                    ),
                    start: 193,
                    end: 280,
                    as_str(): "let lo_bits: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 290,
                                    end: 291,
                                    as_str(): "b",
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
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 294,
                                                    end: 298,
                                                    as_str(): "B512",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 300,
                                                end: 304,
                                                as_str(): "from",
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
                                                    src (ptr): 0x00007fb1013231c0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                    ),
                                                    start: 656,
                                                    end: 666,
                                                    as_str(): "components",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Tuple {
                                                    fields: [
                                                        TyExpression {
                                                            expression: VariableExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 105,
                                                                        end: 112,
                                                                        as_str(): "hi_bits",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 306,
                                                                    end: 313,
                                                                    as_str(): "hi_bits",
                                                                },
                                                                mutability: Immutable,
                                                            },
                                                            return_type: TypeId(
                                                                59,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 306,
                                                                end: 313,
                                                                as_str(): "hi_bits",
                                                            },
                                                        },
                                                        TyExpression {
                                                            expression: VariableExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 197,
                                                                        end: 204,
                                                                        as_str(): "lo_bits",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 315,
                                                                    end: 322,
                                                                    as_str(): "lo_bits",
                                                                },
                                                                mutability: Immutable,
                                                            },
                                                            return_type: TypeId(
                                                                59,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 315,
                                                                end: 322,
                                                                as_str(): "lo_bits",
                                                            },
                                                        },
                                                    ],
                                                },
                                                return_type: TypeId(
                                                    31636,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 305,
                                                    end: 323,
                                                    as_str(): "(hi_bits, lo_bits)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13314,
                                        Span {
                                            src (ptr): 0x00007fb1013231c0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                            ),
                                            start: 648,
                                            end: 771,
                                            as_str(): "fn from(components: (b256, b256)) -> B512 {\n        B512 {\n            bytes: [components.0, components.1],\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 294,
                                                end: 304,
                                                as_str(): "B512::from",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    8978,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 294,
                                    end: 324,
                                    as_str(): "B512::from((hi_bits, lo_bits))",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                8978,
                            ),
                            type_ascription: TypeId(
                                31631,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f57bed10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                    ),
                    start: 286,
                    end: 325,
                    as_str(): "let b = B512::from((hi_bits, lo_bits));",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 334,
                                    end: 341,
                                    as_str(): "other_b",
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
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 344,
                                                    end: 348,
                                                    as_str(): "B512",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 350,
                                                end: 353,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        13315,
                                        Span {
                                            src (ptr): 0x00007fb1013231c0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                            ),
                                            start: 950,
                                            end: 1046,
                                            as_str(): "pub fn new() -> B512 {\n        B512 {\n            bytes: [ZERO_B256, ZERO_B256],\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 344,
                                                end: 353,
                                                as_str(): "B512::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    8978,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 344,
                                    end: 355,
                                    as_str(): "B512::new()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                8978,
                            ),
                            type_ascription: TypeId(
                                31638,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f57bed10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                    ),
                    start: 330,
                    end: 356,
                    as_str(): "let other_b = B512::new();",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: LazyOperator {
                            op: And,
                            lhs: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 375,
                                                    end: 377,
                                                    as_str(): "!=",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 375,
                                                    end: 377,
                                                    as_str(): "!=",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "neq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 375,
                                                end: 377,
                                                as_str(): "!=",
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
                                                    src (ptr): 0x00007fb1013efe80,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2832,
                                                    end: 2836,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: ArrayIndex {
                                                    prefix: TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 290,
                                                                            end: 291,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 363,
                                                                        end: 364,
                                                                        as_str(): "b",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    8978,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 363,
                                                                    end: 364,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1013231c0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                        ),
                                                                        start: 359,
                                                                        end: 364,
                                                                        as_str(): "bytes",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    8976,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    8975,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1013231c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 359,
                                                                    end: 375,
                                                                    as_str(): "bytes: [b256; 2]",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb1013231c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 366,
                                                                    end: 375,
                                                                    as_str(): "[b256; 2]",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 365,
                                                                end: 370,
                                                                as_str(): "bytes",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                8978,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31644,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 363,
                                                            end: 370,
                                                            as_str(): "b.bytes",
                                                        },
                                                    },
                                                    index: TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                0,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            31645,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 372,
                                                            end: 373,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    59,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 362,
                                                    end: 374,
                                                    as_str(): "(b.bytes)[0]",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1013efe80,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2838,
                                                    end: 2843,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: ArrayIndex {
                                                    prefix: TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 334,
                                                                            end: 341,
                                                                            as_str(): "other_b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 379,
                                                                        end: 386,
                                                                        as_str(): "other_b",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    8978,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 379,
                                                                    end: 386,
                                                                    as_str(): "other_b",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1013231c0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                        ),
                                                                        start: 359,
                                                                        end: 364,
                                                                        as_str(): "bytes",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    8976,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    8975,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1013231c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 359,
                                                                    end: 375,
                                                                    as_str(): "bytes: [b256; 2]",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb1013231c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 366,
                                                                    end: 375,
                                                                    as_str(): "[b256; 2]",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 387,
                                                                end: 392,
                                                                as_str(): "bytes",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                8978,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31650,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 379,
                                                            end: 392,
                                                            as_str(): "other_b.bytes",
                                                        },
                                                    },
                                                    index: TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                0,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            31651,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 394,
                                                            end: 395,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    59,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 378,
                                                    end: 396,
                                                    as_str(): "(other_b.bytes)[0]",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13316,
                                        Span {
                                            src (ptr): 0x00007fb1013efe80,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 2825,
                                            end: 2897,
                                            as_str(): "fn neq(self, other: Self) -> bool {\n        (self.eq(other)).not()\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 375,
                                                end: 377,
                                                as_str(): "!=",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 362,
                                    end: 396,
                                    as_str(): "(b.bytes)[0] != (other_b.bytes)[0]",
                                },
                            },
                            rhs: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 415,
                                                    end: 417,
                                                    as_str(): "==",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 415,
                                                    end: 417,
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
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 415,
                                                end: 417,
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
                                                    src (ptr): 0x00007fb1013efe80,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3390,
                                                    end: 3394,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: ArrayIndex {
                                                    prefix: TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 290,
                                                                            end: 291,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 403,
                                                                        end: 404,
                                                                        as_str(): "b",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    8978,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 403,
                                                                    end: 404,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1013231c0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                        ),
                                                                        start: 359,
                                                                        end: 364,
                                                                        as_str(): "bytes",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    8976,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    8975,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1013231c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 359,
                                                                    end: 375,
                                                                    as_str(): "bytes: [b256; 2]",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb1013231c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 366,
                                                                    end: 375,
                                                                    as_str(): "[b256; 2]",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 405,
                                                                end: 410,
                                                                as_str(): "bytes",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                8978,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31656,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 403,
                                                            end: 410,
                                                            as_str(): "b.bytes",
                                                        },
                                                    },
                                                    index: TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            31657,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 412,
                                                            end: 413,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    59,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 402,
                                                    end: 414,
                                                    as_str(): "(b.bytes)[1]",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1013efe80,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3396,
                                                    end: 3401,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: ArrayIndex {
                                                    prefix: TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 334,
                                                                            end: 341,
                                                                            as_str(): "other_b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 419,
                                                                        end: 426,
                                                                        as_str(): "other_b",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    8978,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 419,
                                                                    end: 426,
                                                                    as_str(): "other_b",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1013231c0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                        ),
                                                                        start: 359,
                                                                        end: 364,
                                                                        as_str(): "bytes",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    8976,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    8975,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1013231c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 359,
                                                                    end: 375,
                                                                    as_str(): "bytes: [b256; 2]",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb1013231c0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/b512.sw",
                                                                    ),
                                                                    start: 366,
                                                                    end: 375,
                                                                    as_str(): "[b256; 2]",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 427,
                                                                end: 432,
                                                                as_str(): "bytes",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                8978,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31662,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 419,
                                                            end: 432,
                                                            as_str(): "other_b.bytes",
                                                        },
                                                    },
                                                    index: TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            31663,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 434,
                                                            end: 435,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    59,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 418,
                                                    end: 436,
                                                    as_str(): "(other_b.bytes)[1]",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13317,
                                        Span {
                                            src (ptr): 0x00007fb1013efe80,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3384,
                                            end: 3636,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        // Both self and other are addresses of the values, so we can use MEQ.\n        asm(r1: self, r2: other, r3, r4) {\n            addi r3 zero i32;\n            meq r4 r1 r2 r3;\n            r4: bool\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 415,
                                                end: 417,
                                                as_str(): "==",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f57bed10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                    ),
                                    start: 402,
                                    end: 436,
                                    as_str(): "(b.bytes)[1] == (other_b.bytes)[1]",
                                },
                            },
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 361,
                            end: 437,
                            as_str(): "((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0f57bed10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                    ),
                    start: 361,
                    end: 437,
                    as_str(): "((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb0f57bed10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
        ),
        start: 77,
        end: 439,
        as_str(): "fn main() -> bool {\n    let hi_bits: b256 = 0x7777777777777777777777777777777777777777777777777777777777777777;\n    let lo_bits: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;\n\n    let b = B512::from((hi_bits, lo_bits));\n    let other_b = B512::new();\n    ((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])\n}",
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
        src (ptr): 0x00007fb0f57bed10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
        ),
        start: 90,
        end: 94,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

