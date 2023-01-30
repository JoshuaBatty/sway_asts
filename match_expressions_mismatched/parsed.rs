[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0a2c66060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                            ),
                            start: 16,
                            end: 24,
                            as_str(): "MyStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 27,
                                    end: 28,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 27,
                                end: 33,
                                as_str(): "a: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 30,
                                end: 33,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 35,
                                    end: 36,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 35,
                                end: 41,
                                as_str(): "b: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 38,
                                end: 41,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0a2c66060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                        ),
                        start: 9,
                        end: 43,
                        as_str(): "struct MyStruct { a: u64, b: u64 }",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a2c66060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
            ),
            start: 9,
            end: 43,
            as_str(): "struct MyStruct { a: u64, b: u64 }",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0a2c66060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                            ),
                            start: 50,
                            end: 56,
                            as_str(): "MyEnum",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 61,
                                    end: 69,
                                    as_str(): "Variant1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 71,
                                end: 73,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 61,
                                end: 73,
                                as_str(): "Variant1: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 77,
                                    end: 85,
                                    as_str(): "Variant2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 87,
                                end: 90,
                                as_str(): "u64",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 77,
                                end: 90,
                                as_str(): "Variant2: u64",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 102,
                                    as_str(): "Variant3",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0a2c66060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                        ),
                                        start: 104,
                                        end: 112,
                                        as_str(): "MyStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 104,
                                end: 112,
                                as_str(): "MyStruct",
                            },
                            tag: 2,
                            span: Span {
                                src (ptr): 0x00007fe0a2c66060,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                ),
                                start: 94,
                                end: 112,
                                as_str(): "Variant3: MyStruct",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0a2c66060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                        ),
                        start: 45,
                        end: 115,
                        as_str(): "enum MyEnum {\n  Variant1: (),\n  Variant2: u64,\n  Variant3: MyStruct,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a2c66060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
            ),
            start: 45,
            end: 115,
            as_str(): "enum MyEnum {\n  Variant1: (),\n  Variant2: u64,\n  Variant3: MyStruct,\n}",
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
                            src (ptr): 0x00007fe0a2c66060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                            ),
                            start: 120,
                            end: 124,
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
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 142,
                                                    end: 143,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: DelineatedPath(
                                                    DelineatedPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 146,
                                                                            end: 152,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                        ),
                                                                        start: 154,
                                                                        end: 162,
                                                                        as_str(): "Variant1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 154,
                                                                end: 162,
                                                                as_str(): "Variant1",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 146,
                                                    end: 162,
                                                    as_str(): "MyEnum::Variant1",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 138,
                                    end: 163,
                                    as_str(): "let x = MyEnum::Variant1;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 170,
                                                    end: 171,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 174,
                                                                                end: 180,
                                                                                as_str(): "MyEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 174,
                                                                            end: 180,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 182,
                                                                            end: 190,
                                                                            as_str(): "Variant2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 174,
                                                                end: 190,
                                                                as_str(): "MyEnum::Variant2",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        5,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                    ),
                                                                    start: 193,
                                                                    end: 194,
                                                                    as_str(): "5",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 196,
                                                    as_str(): "MyEnum::Variant2 ( 5 )",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 166,
                                    end: 198,
                                    as_str(): "let y = MyEnum::Variant2 ( 5 ) ;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 205,
                                                    end: 206,
                                                    as_str(): "z",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 209,
                                                                                end: 215,
                                                                                as_str(): "MyEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 209,
                                                                            end: 215,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 217,
                                                                            end: 225,
                                                                            as_str(): "Variant3",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a2c66060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                ),
                                                                start: 209,
                                                                end: 225,
                                                                as_str(): "MyEnum::Variant3",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Struct(
                                                                    StructExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 228,
                                                                                        end: 236,
                                                                                        as_str(): "MyStruct",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 228,
                                                                                end: 236,
                                                                                as_str(): "MyStruct",
                                                                            },
                                                                        },
                                                                        fields: [
                                                                            StructExpressionField {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 239,
                                                                                        end: 240,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 242,
                                                                                        end: 243,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 239,
                                                                                    end: 243,
                                                                                    as_str(): "a: 0",
                                                                                },
                                                                            },
                                                                            StructExpressionField {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 245,
                                                                                        end: 246,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                value: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            1,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 248,
                                                                                        end: 249,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 245,
                                                                                    end: 249,
                                                                                    as_str(): "b: 1",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                    ),
                                                                    start: 228,
                                                                    end: 251,
                                                                    as_str(): "MyStruct { a: 0, b: 1 }",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 209,
                                                    end: 253,
                                                    as_str(): "MyEnum::Variant3 ( MyStruct { a: 0, b: 1 } )",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 201,
                                    end: 255,
                                    as_str(): "let z = MyEnum::Variant3 ( MyStruct { a: 0, b: 1 } ) ;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "__match_return_var_name_1",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 266,
                                                                            as_str(): "y",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 265,
                                                                                    end: 266,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 266,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 318,
                                                            as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                                                        },
                                                    },
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "__match_return_var_name_1",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 265,
                                                                                        end: 266,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                ),
                                                                                start: 265,
                                                                                end: 266,
                                                                                as_str(): "y",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                    ),
                                                                                                    start: 273,
                                                                                                    end: 279,
                                                                                                    as_str(): "MyEnum",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                ),
                                                                                                start: 281,
                                                                                                end: 289,
                                                                                                as_str(): "Variant2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Variable {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                ),
                                                                                                start: 292,
                                                                                                end: 293,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a2c66060,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                            ),
                                                                                            start: 292,
                                                                                            end: 293,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 273,
                                                                                        end: 295,
                                                                                        as_str(): "MyEnum::Variant2 ( y )",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a2c66060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                                ),
                                                                                                start: 299,
                                                                                                end: 300,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 299,
                                                                                        end: 300,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 273,
                                                                                    end: 301,
                                                                                    as_str(): "MyEnum::Variant2 ( y ) => y,",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: CatchAll {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 306,
                                                                                        end: 307,
                                                                                        as_str(): "_",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            10,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a2c66060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                        ),
                                                                                        start: 311,
                                                                                        end: 313,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                                    ),
                                                                                    start: 306,
                                                                                    end: 314,
                                                                                    as_str(): "_ => 10,",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a2c66060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                                    ),
                                                                    start: 259,
                                                                    end: 318,
                                                                    as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a2c66060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 318,
                                                            as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe0a2c66060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                                    ),
                                                    start: 259,
                                                    end: 318,
                                                    as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0a2c66060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                            ),
                                            start: 259,
                                            end: 318,
                                            as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a2c66060,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                                    ),
                                    start: 259,
                                    end: 318,
                                    as_str(): "match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0a2c66060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                            ),
                            start: 134,
                            end: 320,
                            as_str(): "{\n  let x = MyEnum::Variant1;\n  let y = MyEnum::Variant2 ( 5 ) ;\n  let z = MyEnum::Variant3 ( MyStruct { a: 0, b: 1 } ) ;\n\n  match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0a2c66060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                        ),
                        start: 117,
                        end: 320,
                        as_str(): "fn main() -> u64 {\n  let x = MyEnum::Variant1;\n  let y = MyEnum::Variant2 ( 5 ) ;\n  let z = MyEnum::Variant3 ( MyStruct { a: 0, b: 1 } ) ;\n\n  match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0a2c66060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
                        ),
                        start: 130,
                        end: 133,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a2c66060,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRgSO9ZQ/match_expressions_mismatched/src/main.sw",
            ),
            start: 117,
            end: 320,
            as_str(): "fn main() -> u64 {\n  let x = MyEnum::Variant1;\n  let y = MyEnum::Variant2 ( 5 ) ;\n  let z = MyEnum::Variant3 ( MyStruct { a: 0, b: 1 } ) ;\n\n  match y {\n    MyEnum::Variant2 ( y ) => y,\n    _ => 10,\n  }\n}",
        },
    },
]
