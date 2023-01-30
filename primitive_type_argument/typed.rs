
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe05b2d99e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
            ),
            start: 27,
            end: 28,
            as_str(): "S",
        },
        is_raw_ident: false,
    },
    fields: [],
    type_parameters: [
        T: TypeId(31665),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe05b2d99e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
        ),
        start: 20,
        end: 35,
        as_str(): "struct S<T> { }",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13318,
        Span {
            src (ptr): 0x00007fe05b2d99e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
            ),
            start: 37,
            end: 85,
            as_str(): "impl<T> S<T> {\n  fn f(self) -> u64 {\n    5\n  }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe05b2d99e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
            ),
            start: 90,
            end: 94,
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
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 112,
                                    end: 113,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 27,
                                            end: 28,
                                            as_str(): "S",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [],
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 116,
                                        end: 124,
                                        as_str(): "S::<u64>",
                                    },
                                },
                                return_type: TypeId(
                                    31698,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 116,
                                    end: 128,
                                    as_str(): "S::<u64> { }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31698,
                            ),
                            type_ascription: TypeId(
                                31696,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe05b2d99e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                    ),
                    start: 108,
                    end: 129,
                    as_str(): "let a = S::<u64> { };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 136,
                                    end: 137,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05b1b2b70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                            ),
                                            start: 36,
                                            end: 49,
                                            as_str(): "ExampleStruct",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 179,
                                                    end: 186,
                                                    as_str(): "a_field",
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
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 192,
                                                    as_str(): "5u64",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 194,
                                                    end: 201,
                                                    as_str(): "b_field",
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
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 203,
                                                    end: 207,
                                                    as_str(): "true",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 150,
                                        end: 176,
                                        as_str(): "ExampleStruct::<u64, bool>",
                                    },
                                },
                                return_type: TypeId(
                                    31703,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 140,
                                    end: 209,
                                    as_str(): "foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31703,
                            ),
                            type_ascription: TypeId(
                                31701,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe05b2d99e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                    ),
                    start: 132,
                    end: 210,
                    as_str(): "let b = foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true };",
                },
            },
            TyAstNode {
                content: SideEffect,
                span: Span {
                    src (ptr): 0x00007fe05b2d99e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                    ),
                    start: 213,
                    end: 241,
                    as_str(): "use foo::baz::ExampleStruct;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 248,
                                    end: 249,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05b1a68a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                            ),
                                            start: 25,
                                            end: 29,
                                            as_str(): "Quux",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 332,
                                                    end: 333,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 335,
                                                    end: 337,
                                                    as_str(): "10",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 339,
                                                    end: 340,
                                                    as_str(): "b",
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
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 342,
                                                    end: 346,
                                                    as_str(): "true",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 348,
                                                    end: 349,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 136,
                                                            end: 137,
                                                            as_str(): "b",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe05b2d99e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                        ),
                                                        start: 351,
                                                        end: 352,
                                                        as_str(): "b",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    31703,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 351,
                                                    end: 352,
                                                    as_str(): "b",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 354,
                                                    end: 355,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 357,
                                                    end: 359,
                                                    as_str(): "10",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 361,
                                                    end: 362,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    String(
                                                        Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 365,
                                                            end: 368,
                                                            as_str(): "foo",
                                                        },
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    31718,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 364,
                                                    end: 369,
                                                    as_str(): "\"foo\"",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 371,
                                                    end: 372,
                                                    as_str(): "f",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 374,
                                                    end: 376,
                                                    as_str(): "10",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 268,
                                        end: 329,
                                        as_str(): "Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64>",
                                    },
                                },
                                return_type: TypeId(
                                    31710,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 252,
                                    end: 378,
                                    as_str(): "foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31710,
                            ),
                            type_ascription: TypeId(
                                31707,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe05b2d99e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                    ),
                    start: 244,
                    end: 379,
                    as_str(): "let c = foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 };",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe05b2d99e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                ),
                                                start: 391,
                                                end: 392,
                                                as_str(): "f",
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
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 59,
                                                    end: 63,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 113,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe05b2d99e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                        ),
                                                        start: 389,
                                                        end: 390,
                                                        as_str(): "a",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    31698,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 389,
                                                    end: 390,
                                                    as_str(): "a",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13320,
                                        Span {
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 54,
                                            end: 83,
                                            as_str(): "fn f(self) -> u64 {\n    5\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe05b2d99e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                ),
                                                start: 391,
                                                end: 392,
                                                as_str(): "f",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05b2d99e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                    ),
                                    start: 389,
                                    end: 394,
                                    as_str(): "a.f()",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31724,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe05b2d99e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                            ),
                            start: 382,
                            end: 394,
                            as_str(): "return a.f()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe05b2d99e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                    ),
                    start: 382,
                    end: 394,
                    as_str(): "return a.f()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05b2d99e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
        ),
        start: 87,
        end: 397,
        as_str(): "fn main() -> u64 {\n  let a = S::<u64> { };\n  let b = foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true };\n  use foo::baz::ExampleStruct;\n  let c = foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 };\n  return a.f();\n}",
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
        src (ptr): 0x00007fe05b2d99e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
        ),
        start: 100,
        end: 103,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

