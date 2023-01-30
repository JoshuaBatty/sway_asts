



TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0de482d80,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
            ),
            start: 41,
            end: 45,
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
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 66,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0de9b8690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                            ),
                                            start: 23,
                                            end: 24,
                                            as_str(): "B",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 81,
                                                    end: 82,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 84,
                                                    end: 85,
                                                    as_str(): "0",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0de482d80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 70,
                                        as_str(): "B",
                                    },
                                },
                                return_type: TypeId(
                                    31632,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 69,
                                    end: 92,
                                    as_str(): "B {\n        b: 0,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31632,
                            ),
                            type_ascription: TypeId(
                                31636,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0de482d80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                    ),
                    start: 61,
                    end: 93,
                    as_str(): "let x = B {\n        b: 0,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 102,
                                    end: 103,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0de9b8690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                            ),
                                            start: 52,
                                            end: 53,
                                            as_str(): "C",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 118,
                                                    end: 119,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 121,
                                                    end: 122,
                                                    as_str(): "0",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0de482d80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                        ),
                                        start: 106,
                                        end: 107,
                                        as_str(): "C",
                                    },
                                },
                                return_type: TypeId(
                                    31633,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 106,
                                    end: 129,
                                    as_str(): "C {\n        c: 0,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31633,
                            ),
                            type_ascription: TypeId(
                                31640,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0de482d80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                    ),
                    start: 98,
                    end: 130,
                    as_str(): "let y = C {\n        c: 0,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 139,
                                    end: 140,
                                    as_str(): "z",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0de9b8690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                            ),
                                            start: 81,
                                            end: 82,
                                            as_str(): "D",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 155,
                                                    end: 156,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 158,
                                                    end: 159,
                                                    as_str(): "0",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0de482d80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                        ),
                                        start: 143,
                                        end: 144,
                                        as_str(): "D",
                                    },
                                },
                                return_type: TypeId(
                                    31634,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 143,
                                    end: 166,
                                    as_str(): "D {\n        d: 0,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31634,
                            ),
                            type_ascription: TypeId(
                                31644,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0de482d80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                    ),
                    start: 135,
                    end: 167,
                    as_str(): "let z = D {\n        d: 0,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 176,
                                    end: 179,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 192,
                                                    end: 193,
                                                    as_str(): "+",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 192,
                                                    end: 193,
                                                    as_str(): "+",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "add",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0de482d80,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                ),
                                                start: 192,
                                                end: 193,
                                                as_str(): "+",
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
                                                    src (ptr): 0x00007fe0e6ccc350,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 124,
                                                    end: 128,
                                                    as_str(): "self",
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
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 186,
                                                                    end: 187,
                                                                    as_str(): "+",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 186,
                                                                    end: 187,
                                                                    as_str(): "+",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: Some(
                                                                "add",
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 187,
                                                                as_str(): "+",
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
                                                                    src (ptr): 0x00007fe0e6ccc350,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 124,
                                                                    end: 128,
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
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 65,
                                                                                    end: 66,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 182,
                                                                                end: 183,
                                                                                as_str(): "x",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31632,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0de482d80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                            ),
                                                                            start: 182,
                                                                            end: 183,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    field_to_access: TyStructField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de9b8690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                ),
                                                                                start: 31,
                                                                                end: 32,
                                                                                as_str(): "b",
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
                                                                            src (ptr): 0x00007fe0de9b8690,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                            ),
                                                                            start: 31,
                                                                            end: 37,
                                                                            as_str(): "b: u64",
                                                                        },
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fe0de9b8690,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                            ),
                                                                            start: 34,
                                                                            end: 37,
                                                                            as_str(): "u64",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    field_instantiation_span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 184,
                                                                        end: 185,
                                                                        as_str(): "b",
                                                                    },
                                                                    resolved_type_of_parent: TypeId(
                                                                        31632,
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 182,
                                                                    end: 185,
                                                                    as_str(): "x.b",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6ccc350,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 130,
                                                                    end: 135,
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
                                                                                    src (ptr): 0x00007fe0de482d80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                    ),
                                                                                    start: 102,
                                                                                    end: 103,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de482d80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                                ),
                                                                                start: 188,
                                                                                end: 189,
                                                                                as_str(): "y",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31633,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0de482d80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                            ),
                                                                            start: 188,
                                                                            end: 189,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                    field_to_access: TyStructField {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0de9b8690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                                ),
                                                                                start: 60,
                                                                                end: 61,
                                                                                as_str(): "c",
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
                                                                            src (ptr): 0x00007fe0de9b8690,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                            ),
                                                                            start: 60,
                                                                            end: 66,
                                                                            as_str(): "c: u64",
                                                                        },
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fe0de9b8690,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 66,
                                                                            as_str(): "u64",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                    field_instantiation_span: Span {
                                                                        src (ptr): 0x00007fe0de482d80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                        ),
                                                                        start: 190,
                                                                        end: 191,
                                                                        as_str(): "c",
                                                                    },
                                                                    resolved_type_of_parent: TypeId(
                                                                        31633,
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 188,
                                                                    end: 191,
                                                                    as_str(): "y.c",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13317,
                                                        Span {
                                                            src (ptr): 0x00007fe0e6ccc350,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 117,
                                                            end: 185,
                                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 187,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 182,
                                                    end: 191,
                                                    as_str(): "x.b + y.c",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0e6ccc350,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 130,
                                                    end: 135,
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
                                                                    src (ptr): 0x00007fe0de482d80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                    ),
                                                                    start: 139,
                                                                    end: 140,
                                                                    as_str(): "z",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de482d80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                                ),
                                                                start: 194,
                                                                end: 195,
                                                                as_str(): "z",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31634,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0de482d80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 195,
                                                            as_str(): "z",
                                                        },
                                                    },
                                                    field_to_access: TyStructField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0de9b8690,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                                ),
                                                                start: 89,
                                                                end: 90,
                                                                as_str(): "d",
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
                                                            src (ptr): 0x00007fe0de9b8690,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                            ),
                                                            start: 89,
                                                            end: 95,
                                                            as_str(): "d: u64",
                                                        },
                                                        type_span: Span {
                                                            src (ptr): 0x00007fe0de9b8690,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/lib.sw",
                                                            ),
                                                            start: 92,
                                                            end: 95,
                                                            as_str(): "u64",
                                                        },
                                                        attributes: {},
                                                    },
                                                    field_instantiation_span: Span {
                                                        src (ptr): 0x00007fe0de482d80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                        ),
                                                        start: 196,
                                                        end: 197,
                                                        as_str(): "d",
                                                    },
                                                    resolved_type_of_parent: TypeId(
                                                        31634,
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0de482d80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                    ),
                                                    start: 194,
                                                    end: 197,
                                                    as_str(): "z.d",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13318,
                                        Span {
                                            src (ptr): 0x00007fe0e6ccc350,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 117,
                                            end: 185,
                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0de482d80,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                                ),
                                                start: 192,
                                                end: 193,
                                                as_str(): "+",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 182,
                                    end: 197,
                                    as_str(): "x.b + y.c + z.d",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31648,
                            ),
                            type_ascription: TypeId(
                                31648,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0de482d80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                    ),
                    start: 172,
                    end: 198,
                    as_str(): "let foo = x.b + y.c + z.d;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0de482d80,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                    ),
                                    start: 176,
                                    end: 179,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe0de482d80,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                                ),
                                start: 203,
                                end: 206,
                                as_str(): "foo",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            31648,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0de482d80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                            ),
                            start: 203,
                            end: 206,
                            as_str(): "foo",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0de482d80,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
                    ),
                    start: 203,
                    end: 206,
                    as_str(): "foo",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0de482d80,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
        ),
        start: 38,
        end: 208,
        as_str(): "fn main() -> u64 {\n    let x = B {\n        b: 0,\n    };\n    let y = C {\n        c: 0,\n    };\n    let z = D {\n        d: 0,\n    };\n    let foo = x.b + y.c + z.d;\n    foo\n}",
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
        src (ptr): 0x00007fe0de482d80,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRsGc8FE/import_trailing_comma/src/main.sw",
        ),
        start: 51,
        end: 54,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

