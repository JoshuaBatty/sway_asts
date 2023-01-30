





TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe08b1f4f00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
            ),
            start: 156,
            end: 160,
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
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 181,
                                    end: 185,
                                    as_str(): "bar1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0933fb3a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                            ),
                                            start: 47,
                                            end: 51,
                                            as_str(): "Bar1",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 205,
                                                    end: 206,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 208,
                                                    end: 212,
                                                    as_str(): "5u32",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 188,
                                        end: 194,
                                        as_str(): "MyBar1",
                                    },
                                },
                                return_type: TypeId(
                                    7255,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 188,
                                    end: 219,
                                    as_str(): "MyBar1 {\n        a: 5u32,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7255,
                            ),
                            type_ascription: TypeId(
                                7261,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe08b1f4f00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                    ),
                    start: 177,
                    end: 220,
                    as_str(): "let bar1 = MyBar1 {\n        a: 5u32,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 229,
                                    end: 233,
                                    as_str(): "bar2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0933fb3a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                            ),
                                            start: 80,
                                            end: 84,
                                            as_str(): "Bar2",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 251,
                                                    end: 252,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 254,
                                                    end: 258,
                                                    as_str(): "5u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 236,
                                        end: 240,
                                        as_str(): "Bar2",
                                    },
                                },
                                return_type: TypeId(
                                    7256,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 236,
                                    end: 265,
                                    as_str(): "Bar2 {\n        a: 5u64,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7256,
                            ),
                            type_ascription: TypeId(
                                7264,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe08b1f4f00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                    ),
                    start: 225,
                    end: 266,
                    as_str(): "let bar2 = Bar2 {\n        a: 5u64,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 275,
                                    end: 278,
                                    as_str(): "db1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f6cb6e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                            ),
                                            start: 32,
                                            end: 42,
                                            as_str(): "DoubleBar1",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 304,
                                                    end: 305,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 307,
                                                    end: 311,
                                                    as_str(): "5u32",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 281,
                                        end: 293,
                                        as_str(): "MyDoubleBar1",
                                    },
                                },
                                return_type: TypeId(
                                    7257,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 281,
                                    end: 318,
                                    as_str(): "MyDoubleBar1 {\n        a: 5u32,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7257,
                            ),
                            type_ascription: TypeId(
                                7267,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe08b1f4f00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                    ),
                    start: 271,
                    end: 319,
                    as_str(): "let db1 = MyDoubleBar1 {\n        a: 5u32,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 328,
                                    end: 331,
                                    as_str(): "db2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f6cb6e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                            ),
                                            start: 71,
                                            end: 81,
                                            as_str(): "DoubleBar2",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 357,
                                                    end: 358,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 360,
                                                    end: 364,
                                                    as_str(): "5u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 334,
                                        end: 346,
                                        as_str(): "MyDoubleBar2",
                                    },
                                },
                                return_type: TypeId(
                                    7258,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 334,
                                    end: 371,
                                    as_str(): "MyDoubleBar2 {\n        a: 5u64,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7258,
                            ),
                            type_ascription: TypeId(
                                7270,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe08b1f4f00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                    ),
                    start: 324,
                    end: 372,
                    as_str(): "let db2 = MyDoubleBar2 {\n        a: 5u64,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 381,
                                    end: 384,
                                    as_str(): "db3",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f6cb6e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                            ),
                                            start: 110,
                                            end: 120,
                                            as_str(): "DoubleBar3",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 408,
                                                    end: 409,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 411,
                                                    end: 415,
                                                    as_str(): "5u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 387,
                                        end: 397,
                                        as_str(): "DoubleBar3",
                                    },
                                },
                                return_type: TypeId(
                                    7259,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 387,
                                    end: 422,
                                    as_str(): "DoubleBar3 {\n        a: 5u64,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7259,
                            ),
                            type_ascription: TypeId(
                                7273,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe08b1f4f00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                    ),
                    start: 377,
                    end: 423,
                    as_str(): "let db3 = DoubleBar3 {\n        a: 5u64,\n    };",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            Boolean(
                                false,
                            ),
                        ),
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 428,
                            end: 433,
                            as_str(): "false",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe08b1f4f00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                    ),
                    start: 428,
                    end: 433,
                    as_str(): "false",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe08b1f4f00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
        ),
        start: 153,
        end: 435,
        as_str(): "fn main() -> bool {\n    let bar1 = MyBar1 {\n        a: 5u32,\n    };\n    let bar2 = Bar2 {\n        a: 5u64,\n    };\n    let db1 = MyDoubleBar1 {\n        a: 5u32,\n    };\n    let db2 = MyDoubleBar2 {\n        a: 5u64,\n    };\n    let db3 = DoubleBar3 {\n        a: 5u64,\n    };\n    false\n}",
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
        src (ptr): 0x00007fe08b1f4f00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
        ),
        start: 166,
        end: 170,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

