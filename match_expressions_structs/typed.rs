TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe092b73f20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
            ),
            start: 16,
            end: 21,
            as_str(): "Point",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe092b73f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                    ),
                    start: 28,
                    end: 29,
                    as_str(): "x",
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
                src (ptr): 0x00007fe092b73f20,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                ),
                start: 28,
                end: 34,
                as_str(): "x: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe092b73f20,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                ),
                start: 31,
                end: 34,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe092b73f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                    ),
                    start: 40,
                    end: 41,
                    as_str(): "y",
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
                src (ptr): 0x00007fe092b73f20,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                ),
                start: 40,
                end: 46,
                as_str(): "y: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe092b73f20,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                ),
                start: 43,
                end: 46,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe092b73f20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
        ),
        start: 9,
        end: 48,
        as_str(): "struct Point {\n    x: u64,\n    y: u64\n}",
    },
    attributes: {},
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe092b73f20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
            ),
            start: 57,
            end: 61,
            as_str(): "Data",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe092b73f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                    ),
                    start: 71,
                    end: 76,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7252,
            ),
            initial_type_id: TypeId(
                7253,
            ),
            span: Span {
                src (ptr): 0x00007fe092b73f20,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                ),
                start: 71,
                end: 79,
                as_str(): "value: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe092b73f20,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                ),
                start: 78,
                end: 79,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7252),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe092b73f20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
        ),
        start: 50,
        end: 81,
        as_str(): "struct Data<T> {\n    value: T\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe092b73f20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
            ),
            start: 86,
            end: 90,
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
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 110,
                                    end: 111,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe092b73f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 21,
                                            as_str(): "Point",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 130,
                                                    end: 131,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        3,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 134,
                                                    as_str(): "3",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 144,
                                                    end: 145,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 147,
                                                    end: 148,
                                                    as_str(): "4",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 114,
                                        end: 119,
                                        as_str(): "Point",
                                    },
                                },
                                return_type: TypeId(
                                    7257,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 155,
                                    as_str(): "Point {\n        x: 3,\n        y: 4,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7257,
                            ),
                            type_ascription: TypeId(
                                7255,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe092b73f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                    ),
                    start: 106,
                    end: 156,
                    as_str(): "let a = Point {\n        x: 3,\n        y: 4,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 165,
                                    end: 166,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Declaration(
                                                    VariableDeclaration(
                                                        TyVariableDeclaration {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "__match_return_var_name_1",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 175,
                                                                    end: 176,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 110,
                                                                            end: 111,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 176,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7257,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 175,
                                                                    end: 176,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7257,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7263,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 281,
                                                    as_str(): "match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                                },
                                            },
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: IfExp {
                                                            condition: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 195,
                                                                                    end: 199,
                                                                                    as_str(): "x: 3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 195,
                                                                                    end: 199,
                                                                                    as_str(): "x: 3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 195,
                                                                                end: 199,
                                                                                as_str(): "x: 3",
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
                                                                                    src (ptr): 0x00007fe0ae7ab2c0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3022,
                                                                                    end: 3026,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: StructFieldAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_1",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 175,
                                                                                                    end: 176,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 175,
                                                                                                end: 176,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7257,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 175,
                                                                                            end: 176,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: TyStructField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 28,
                                                                                                end: 29,
                                                                                                as_str(): "x",
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
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 28,
                                                                                            end: 34,
                                                                                            as_str(): "x: u64",
                                                                                        },
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 31,
                                                                                            end: 34,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    field_instantiation_span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 195,
                                                                                        end: 196,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7257,
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 195,
                                                                                    end: 199,
                                                                                    as_str(): "x: 3",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ae7ab2c0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 3028,
                                                                                    end: 3033,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    Numeric(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 198,
                                                                                    end: 199,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        548,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0ae7ab2c0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 3016,
                                                                            end: 3082,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 195,
                                                                    end: 199,
                                                                    as_str(): "x: 3",
                                                                },
                                                            },
                                                            then: TyExpression {
                                                                expression: CodeBlock(
                                                                    TyCodeBlock {
                                                                        contents: [
                                                                            TyAstNode {
                                                                                content: Declaration(
                                                                                    VariableDeclaration(
                                                                                        TyVariableDeclaration {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 201,
                                                                                                    end: 202,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            body: TyExpression {
                                                                                                expression: StructFieldAccess {
                                                                                                    prefix: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "__match_return_var_name_1",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 175,
                                                                                                                    end: 176,
                                                                                                                    as_str(): "a",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 175,
                                                                                                                end: 176,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            7257,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 175,
                                                                                                            end: 176,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                    },
                                                                                                    field_to_access: TyStructField {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 40,
                                                                                                                end: 41,
                                                                                                                as_str(): "y",
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
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 40,
                                                                                                            end: 46,
                                                                                                            as_str(): "y: u64",
                                                                                                        },
                                                                                                        type_span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 43,
                                                                                                            end: 46,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                        attributes: {},
                                                                                                    },
                                                                                                    field_instantiation_span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 201,
                                                                                                        end: 202,
                                                                                                        as_str(): "y",
                                                                                                    },
                                                                                                    resolved_type_of_parent: TypeId(
                                                                                                        7257,
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 201,
                                                                                                    end: 202,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            type_ascription: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            type_ascription_span: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 201,
                                                                                    end: 202,
                                                                                    as_str(): "y",
                                                                                },
                                                                            },
                                                                            TyAstNode {
                                                                                content: ImplicitReturnExpression(
                                                                                    TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 201,
                                                                                                    end: 202,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 210,
                                                                                                end: 211,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 210,
                                                                                            end: 211,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 210,
                                                                                    end: 211,
                                                                                    as_str(): "y",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 208,
                                                                    end: 213,
                                                                    as_str(): "{ y }",
                                                                },
                                                            },
                                                            else: Some(
                                                                TyExpression {
                                                                    expression: IfExp {
                                                                        condition: TyExpression {
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
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 231,
                                                                                                        end: 235,
                                                                                                        as_str(): "x: 3",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 231,
                                                                                                        end: 235,
                                                                                                        as_str(): "x: 3",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "eq",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 231,
                                                                                                    end: 235,
                                                                                                    as_str(): "x: 3",
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
                                                                                                        src (ptr): 0x00007fe0ae7ab2c0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 3022,
                                                                                                        end: 3026,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: StructFieldAccess {
                                                                                                        prefix: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 175,
                                                                                                                        end: 176,
                                                                                                                        as_str(): "a",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 175,
                                                                                                                    end: 176,
                                                                                                                    as_str(): "a",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7257,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 175,
                                                                                                                end: 176,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: TyStructField {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 28,
                                                                                                                    end: 29,
                                                                                                                    as_str(): "x",
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
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 28,
                                                                                                                end: 34,
                                                                                                                as_str(): "x: u64",
                                                                                                            },
                                                                                                            type_span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 31,
                                                                                                                end: 34,
                                                                                                                as_str(): "u64",
                                                                                                            },
                                                                                                            attributes: {},
                                                                                                        },
                                                                                                        field_instantiation_span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 231,
                                                                                                            end: 232,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        resolved_type_of_parent: TypeId(
                                                                                                            7257,
                                                                                                        ),
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 231,
                                                                                                        end: 235,
                                                                                                        as_str(): "x: 3",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ae7ab2c0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 3028,
                                                                                                        end: 3033,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: Literal(
                                                                                                        Numeric(
                                                                                                            3,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 234,
                                                                                                        end: 235,
                                                                                                        as_str(): "3",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            547,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe0ae7ab2c0,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                ),
                                                                                                start: 3016,
                                                                                                end: 3082,
                                                                                                as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: None,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 231,
                                                                                        end: 235,
                                                                                        as_str(): "x: 3",
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
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 237,
                                                                                                        end: 241,
                                                                                                        as_str(): "y: 4",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 237,
                                                                                                        end: 241,
                                                                                                        as_str(): "y: 4",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "eq",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 237,
                                                                                                    end: 241,
                                                                                                    as_str(): "y: 4",
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
                                                                                                        src (ptr): 0x00007fe0ae7ab2c0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 3022,
                                                                                                        end: 3026,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: StructFieldAccess {
                                                                                                        prefix: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 175,
                                                                                                                        end: 176,
                                                                                                                        as_str(): "a",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 175,
                                                                                                                    end: 176,
                                                                                                                    as_str(): "a",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7257,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 175,
                                                                                                                end: 176,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: TyStructField {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 40,
                                                                                                                    end: 41,
                                                                                                                    as_str(): "y",
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
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 40,
                                                                                                                end: 46,
                                                                                                                as_str(): "y: u64",
                                                                                                            },
                                                                                                            type_span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 43,
                                                                                                                end: 46,
                                                                                                                as_str(): "u64",
                                                                                                            },
                                                                                                            attributes: {},
                                                                                                        },
                                                                                                        field_instantiation_span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 237,
                                                                                                            end: 238,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                        resolved_type_of_parent: TypeId(
                                                                                                            7257,
                                                                                                        ),
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 237,
                                                                                                        end: 241,
                                                                                                        as_str(): "y: 4",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ae7ab2c0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                        ),
                                                                                                        start: 3028,
                                                                                                        end: 3033,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                TyExpression {
                                                                                                    expression: Literal(
                                                                                                        Numeric(
                                                                                                            4,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 240,
                                                                                                        end: 241,
                                                                                                        as_str(): "4",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            546,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe0ae7ab2c0,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                ),
                                                                                                start: 3016,
                                                                                                end: 3082,
                                                                                                as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                                            },
                                                                                        ),
                                                                                        self_state_idx: None,
                                                                                        selector: None,
                                                                                        type_binding: None,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 237,
                                                                                        end: 241,
                                                                                        as_str(): "y: 4",
                                                                                    },
                                                                                },
                                                                            },
                                                                            return_type: TypeId(
                                                                                71,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 231,
                                                                                end: 241,
                                                                                as_str(): "x: 3, y: 4",
                                                                            },
                                                                        },
                                                                        then: TyExpression {
                                                                            expression: CodeBlock(
                                                                                TyCodeBlock {
                                                                                    contents: [
                                                                                        TyAstNode {
                                                                                            content: ImplicitReturnExpression(
                                                                                                TyExpression {
                                                                                                    expression: Literal(
                                                                                                        U64(
                                                                                                            24,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 249,
                                                                                                        end: 251,
                                                                                                        as_str(): "24",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 249,
                                                                                                end: 251,
                                                                                                as_str(): "24",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 247,
                                                                                end: 253,
                                                                                as_str(): "{ 24 }",
                                                                            },
                                                                        },
                                                                        else: Some(
                                                                            TyExpression {
                                                                                expression: CodeBlock(
                                                                                    TyCodeBlock {
                                                                                        contents: [
                                                                                            TyAstNode {
                                                                                                content: ImplicitReturnExpression(
                                                                                                    TyExpression {
                                                                                                        expression: Literal(
                                                                                                            U64(
                                                                                                                24,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 270,
                                                                                                            end: 272,
                                                                                                            as_str(): "24",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 270,
                                                                                                    end: 272,
                                                                                                    as_str(): "24",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 268,
                                                                                    end: 274,
                                                                                    as_str(): "{ 24 }",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 247,
                                                                        end: 253,
                                                                        as_str(): "{ 24 }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 208,
                                                            end: 213,
                                                            as_str(): "{ y }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 281,
                                                    as_str(): "match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 169,
                                    end: 281,
                                    as_str(): "match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7262,
                            ),
                            type_ascription: TypeId(
                                7262,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe092b73f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                    ),
                    start: 161,
                    end: 282,
                    as_str(): "let b = match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 292,
                                    end: 293,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe092b73f20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                            ),
                                            start: 57,
                                            end: 61,
                                            as_str(): "Data",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 311,
                                                    end: 316,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 318,
                                                    end: 322,
                                                    as_str(): "true",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe092b73f20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                        ),
                                        start: 296,
                                        end: 300,
                                        as_str(): "Data",
                                    },
                                },
                                return_type: TypeId(
                                    7278,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 296,
                                    end: 328,
                                    as_str(): "Data {\n        value: true\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7278,
                            ),
                            type_ascription: TypeId(
                                7275,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe092b73f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                    ),
                    start: 288,
                    end: 329,
                    as_str(): "let c = Data {\n        value: true\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 338,
                                    end: 339,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Declaration(
                                                    VariableDeclaration(
                                                        TyVariableDeclaration {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "__match_return_var_name_2",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 348,
                                                                    end: 349,
                                                                    as_str(): "c",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe092b73f20,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                            ),
                                                                            start: 292,
                                                                            end: 293,
                                                                            as_str(): "c",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 348,
                                                                        end: 349,
                                                                        as_str(): "c",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7278,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 348,
                                                                    end: 349,
                                                                    as_str(): "c",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7278,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7281,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 342,
                                                    end: 430,
                                                    as_str(): "match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                                },
                                            },
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: IfExp {
                                                            condition: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 367,
                                                                                    end: 379,
                                                                                    as_str(): "value: false",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 367,
                                                                                    end: 379,
                                                                                    as_str(): "value: false",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                ),
                                                                                start: 367,
                                                                                end: 379,
                                                                                as_str(): "value: false",
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
                                                                                    src (ptr): 0x00007fe0ae7ab2c0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 2930,
                                                                                    end: 2934,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: StructFieldAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_2",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 348,
                                                                                                    end: 349,
                                                                                                    as_str(): "c",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 348,
                                                                                                end: 349,
                                                                                                as_str(): "c",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7278,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 348,
                                                                                            end: 349,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: TyStructField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 71,
                                                                                                end: 76,
                                                                                                as_str(): "value",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_id: TypeId(
                                                                                            7277,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            7253,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 71,
                                                                                            end: 79,
                                                                                            as_str(): "value: T",
                                                                                        },
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 78,
                                                                                            end: 79,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    field_instantiation_span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 367,
                                                                                        end: 372,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7278,
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7277,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 367,
                                                                                    end: 379,
                                                                                    as_str(): "value: false",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ae7ab2c0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 2936,
                                                                                    end: 2941,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: Literal(
                                                                                    Boolean(
                                                                                        false,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    7277,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 374,
                                                                                    end: 379,
                                                                                    as_str(): "false",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        549,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0ae7ab2c0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 2924,
                                                                            end: 2990,
                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: None,
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 367,
                                                                    end: 379,
                                                                    as_str(): "value: false",
                                                                },
                                                            },
                                                            then: TyExpression {
                                                                expression: CodeBlock(
                                                                    TyCodeBlock {
                                                                        contents: [
                                                                            TyAstNode {
                                                                                content: ImplicitReturnExpression(
                                                                                    TyExpression {
                                                                                        expression: Literal(
                                                                                            U64(
                                                                                                0,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 387,
                                                                                            end: 388,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 387,
                                                                                    end: 388,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe092b73f20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                    ),
                                                                    start: 385,
                                                                    end: 390,
                                                                    as_str(): "{ 0 }",
                                                                },
                                                            },
                                                            else: Some(
                                                                TyExpression {
                                                                    expression: CodeBlock(
                                                                        TyCodeBlock {
                                                                            contents: [
                                                                                TyAstNode {
                                                                                    content: Declaration(
                                                                                        VariableDeclaration(
                                                                                            TyVariableDeclaration {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 407,
                                                                                                        end: 412,
                                                                                                        as_str(): "value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                body: TyExpression {
                                                                                                    expression: StructFieldAccess {
                                                                                                        prefix: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_2",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 348,
                                                                                                                        end: 349,
                                                                                                                        as_str(): "c",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 348,
                                                                                                                    end: 349,
                                                                                                                    as_str(): "c",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7278,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 348,
                                                                                                                end: 349,
                                                                                                                as_str(): "c",
                                                                                                            },
                                                                                                        },
                                                                                                        field_to_access: TyStructField {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe092b73f20,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 71,
                                                                                                                    end: 76,
                                                                                                                    as_str(): "value",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_id: TypeId(
                                                                                                                7277,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                7253,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 71,
                                                                                                                end: 79,
                                                                                                                as_str(): "value: T",
                                                                                                            },
                                                                                                            type_span: Span {
                                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 78,
                                                                                                                end: 79,
                                                                                                                as_str(): "T",
                                                                                                            },
                                                                                                            attributes: {},
                                                                                                        },
                                                                                                        field_instantiation_span: Span {
                                                                                                            src (ptr): 0x00007fe092b73f20,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 407,
                                                                                                            end: 412,
                                                                                                            as_str(): "value",
                                                                                                        },
                                                                                                        resolved_type_of_parent: TypeId(
                                                                                                            7278,
                                                                                                        ),
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        7277,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 407,
                                                                                                        end: 412,
                                                                                                        as_str(): "value",
                                                                                                    },
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                                return_type: TypeId(
                                                                                                    7277,
                                                                                                ),
                                                                                                type_ascription: TypeId(
                                                                                                    7277,
                                                                                                ),
                                                                                                type_ascription_span: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 407,
                                                                                        end: 412,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                },
                                                                                TyAstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        TyExpression {
                                                                                            expression: Literal(
                                                                                                U64(
                                                                                                    4,
                                                                                                ),
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe092b73f20,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 420,
                                                                                                end: 421,
                                                                                                as_str(): "4",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe092b73f20,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 420,
                                                                                        end: 421,
                                                                                        as_str(): "4",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe092b73f20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                                        ),
                                                                        start: 418,
                                                                        end: 423,
                                                                        as_str(): "{ 4 }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe092b73f20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                            ),
                                                            start: 385,
                                                            end: 390,
                                                            as_str(): "{ 0 }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe092b73f20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                                    ),
                                                    start: 342,
                                                    end: 430,
                                                    as_str(): "match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 342,
                                    end: 430,
                                    as_str(): "match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7280,
                            ),
                            type_ascription: TypeId(
                                7280,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe092b73f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                    ),
                    start: 334,
                    end: 431,
                    as_str(): "let d = match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    };",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe092b73f20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                    ),
                                    start: 338,
                                    end: 339,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe092b73f20,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                                ),
                                start: 437,
                                end: 438,
                                as_str(): "d",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7280,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe092b73f20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                            ),
                            start: 437,
                            end: 438,
                            as_str(): "d",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe092b73f20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
                    ),
                    start: 437,
                    end: 438,
                    as_str(): "d",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe092b73f20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
        ),
        start: 83,
        end: 440,
        as_str(): "fn main() -> u64 {\n    let a = Point {\n        x: 3,\n        y: 4,\n    };\n    let b = match a {\n        Point { x: 3, y } => { y },\n        Point { x: 3, y: 4 } => { 24 },\n        _ => { 24 },\n    };\n\n    let c = Data {\n        value: true\n    };\n    let d = match c {\n        Data { value: false } => { 0 },\n        Data { value } => { 4 },\n    };\n\n    d\n}",
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
        src (ptr): 0x00007fe092b73f20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRBCSgpv/match_expressions_structs/src/main.sw",
        ),
        start: 96,
        end: 99,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

