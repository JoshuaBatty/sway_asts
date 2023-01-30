[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 18,
                            end: 22,
                            as_str(): "hash",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 24,
                            end: 30,
                            as_str(): "sha256",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 9,
            end: 31,
            as_str(): "use std::hash::sha256;",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 40,
                            end: 48,
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
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 55,
                                    end: 56,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 55,
                                end: 61,
                                as_str(): "x: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 58,
                                end: 61,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 68,
                                    end: 69,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Boolean,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 68,
                                end: 75,
                                as_str(): "y: bool",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 71,
                                end: 75,
                                as_str(): "bool",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 33,
                        end: 77,
                        as_str(): "struct MyStruct {\n    x: u64, \n    y: bool\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 33,
            end: 77,
            as_str(): "struct MyStruct {\n    x: u64, \n    y: bool\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 84,
                            end: 90,
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
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 97,
                                    end: 98,
                                    as_str(): "A",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 100,
                                end: 103,
                                as_str(): "u64",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 97,
                                end: 103,
                                as_str(): "A: u64",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 109,
                                    end: 110,
                                    as_str(): "B",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Boolean,
                            type_span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 112,
                                end: 116,
                                as_str(): "bool",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 109,
                                end: 116,
                                as_str(): "B: bool",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 79,
                        end: 119,
                        as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 79,
            end: 119,
            as_str(): "enum MyEnum {\n    A: u64,\n    B: bool,\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 126,
                                    end: 130,
                                    as_str(): "core",
                                },
                                is_raw_ident: false,
                            },
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 132,
                                    end: 135,
                                    as_str(): "ops",
                                },
                                is_raw_ident: false,
                            },
                        ],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 137,
                                end: 139,
                                as_str(): "Eq",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 144,
                                end: 150,
                                as_str(): "MyEnum",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 144,
                        end: 150,
                        as_str(): "MyEnum",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 160,
                                    end: 162,
                                    as_str(): "eq",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 208,
                                                                                    end: 221,
                                                                                    as_str(): "(self, other)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_ascription: Unknown,
                                                                            type_ascription_span: None,
                                                                            body: Expression {
                                                                                kind: Tuple(
                                                                                    [
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 209,
                                                                                                        end: 213,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 209,
                                                                                                end: 213,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 215,
                                                                                                        end: 220,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 215,
                                                                                                end: 220,
                                                                                                as_str(): "other",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 208,
                                                                                    end: 221,
                                                                                    as_str(): "(self, other)",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 202,
                                                                    end: 403,
                                                                    as_str(): "match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }",
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
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 208,
                                                                                                end: 221,
                                                                                                as_str(): "(self, other)",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 208,
                                                                                        end: 221,
                                                                                        as_str(): "(self, other)",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: Tuple {
                                                                                            elems: [
                                                                                                EnumScrutinee {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 237,
                                                                                                                    end: 243,
                                                                                                                    as_str(): "MyEnum",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 245,
                                                                                                                end: 246,
                                                                                                                as_str(): "A",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: Variable {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 247,
                                                                                                                end: 253,
                                                                                                                as_str(): "inner1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                            ),
                                                                                                            start: 247,
                                                                                                            end: 253,
                                                                                                            as_str(): "inner1",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 237,
                                                                                                        end: 254,
                                                                                                        as_str(): "MyEnum::A(inner1)",
                                                                                                    },
                                                                                                },
                                                                                                EnumScrutinee {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 256,
                                                                                                                    end: 262,
                                                                                                                    as_str(): "MyEnum",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 264,
                                                                                                                end: 265,
                                                                                                                as_str(): "A",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: Variable {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 266,
                                                                                                                end: 272,
                                                                                                                as_str(): "inner2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                            ),
                                                                                                            start: 266,
                                                                                                            end: 272,
                                                                                                            as_str(): "inner2",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 256,
                                                                                                        end: 273,
                                                                                                        as_str(): "MyEnum::A(inner2)",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 236,
                                                                                                end: 274,
                                                                                                as_str(): "(MyEnum::A(inner1), MyEnum::A(inner2))",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: MethodApplication(
                                                                                                MethodApplicationExpression {
                                                                                                    method_name_binding: TypeBinding {
                                                                                                        inner: FromTrait {
                                                                                                            call_path: CallPath {
                                                                                                                prefixes: [
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "core",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 286,
                                                                                                                            end: 288,
                                                                                                                            as_str(): "==",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 286,
                                                                                                                            end: 288,
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
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 286,
                                                                                                                        end: 288,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                is_absolute: true,
                                                                                                            },
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                            ),
                                                                                                            start: 286,
                                                                                                            end: 288,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                    },
                                                                                                    contract_call_params: [],
                                                                                                    arguments: [
                                                                                                        Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 279,
                                                                                                                        end: 285,
                                                                                                                        as_str(): "inner1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 279,
                                                                                                                end: 285,
                                                                                                                as_str(): "inner1",
                                                                                                            },
                                                                                                        },
                                                                                                        Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 289,
                                                                                                                        end: 295,
                                                                                                                        as_str(): "inner2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 289,
                                                                                                                end: 295,
                                                                                                                as_str(): "inner2",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 279,
                                                                                                end: 295,
                                                                                                as_str(): "inner1 == inner2",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 236,
                                                                                            end: 296,
                                                                                            as_str(): "(MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: Tuple {
                                                                                            elems: [
                                                                                                EnumScrutinee {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 310,
                                                                                                                    end: 316,
                                                                                                                    as_str(): "MyEnum",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 318,
                                                                                                                end: 319,
                                                                                                                as_str(): "B",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: Variable {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 320,
                                                                                                                end: 326,
                                                                                                                as_str(): "inner1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                            ),
                                                                                                            start: 320,
                                                                                                            end: 326,
                                                                                                            as_str(): "inner1",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 310,
                                                                                                        end: 327,
                                                                                                        as_str(): "MyEnum::B(inner1)",
                                                                                                    },
                                                                                                },
                                                                                                EnumScrutinee {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 329,
                                                                                                                    end: 335,
                                                                                                                    as_str(): "MyEnum",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 337,
                                                                                                                end: 338,
                                                                                                                as_str(): "B",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: Variable {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 339,
                                                                                                                end: 345,
                                                                                                                as_str(): "inner2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                            ),
                                                                                                            start: 339,
                                                                                                            end: 345,
                                                                                                            as_str(): "inner2",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 329,
                                                                                                        end: 346,
                                                                                                        as_str(): "MyEnum::B(inner2)",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 309,
                                                                                                end: 347,
                                                                                                as_str(): "(MyEnum::B(inner1), MyEnum::B(inner2))",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: MethodApplication(
                                                                                                MethodApplicationExpression {
                                                                                                    method_name_binding: TypeBinding {
                                                                                                        inner: FromTrait {
                                                                                                            call_path: CallPath {
                                                                                                                prefixes: [
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "core",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 359,
                                                                                                                            end: 361,
                                                                                                                            as_str(): "==",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 359,
                                                                                                                            end: 361,
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
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 359,
                                                                                                                        end: 361,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                is_absolute: true,
                                                                                                            },
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                            ),
                                                                                                            start: 359,
                                                                                                            end: 361,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                    },
                                                                                                    contract_call_params: [],
                                                                                                    arguments: [
                                                                                                        Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 352,
                                                                                                                        end: 358,
                                                                                                                        as_str(): "inner1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 352,
                                                                                                                end: 358,
                                                                                                                as_str(): "inner1",
                                                                                                            },
                                                                                                        },
                                                                                                        Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 362,
                                                                                                                        end: 368,
                                                                                                                        as_str(): "inner2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 362,
                                                                                                                end: 368,
                                                                                                                as_str(): "inner2",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 352,
                                                                                                end: 368,
                                                                                                as_str(): "inner1 == inner2",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 309,
                                                                                            end: 369,
                                                                                            as_str(): "(MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 382,
                                                                                                end: 383,
                                                                                                as_str(): "_",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: Literal(
                                                                                                Boolean(
                                                                                                    false,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 387,
                                                                                                end: 392,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 382,
                                                                                            end: 393,
                                                                                            as_str(): "_ => false,",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 202,
                                                                            end: 403,
                                                                            as_str(): "match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 202,
                                                                    end: 403,
                                                                    as_str(): "match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 202,
                                                            end: 403,
                                                            as_str(): "match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 202,
                                                    end: 403,
                                                    as_str(): "match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 202,
                                            end: 403,
                                            as_str(): "match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 192,
                                    end: 409,
                                    as_str(): "{\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 163,
                                            end: 167,
                                            as_str(): "self",
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
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 163,
                                        end: 167,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 169,
                                            end: 174,
                                            as_str(): "other",
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
                                    type_info: Custom {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 176,
                                                end: 182,
                                                as_str(): "MyEnum",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_arguments: Some(
                                            [],
                                        ),
                                    },
                                    type_span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 176,
                                        end: 182,
                                        as_str(): "MyEnum",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 157,
                                end: 409,
                                as_str(): "fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 187,
                                end: 191,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 121,
                        end: 411,
                        as_str(): "impl core::ops::Eq for MyEnum {\n    fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 121,
            end: 411,
            as_str(): "impl core::ops::Eq for MyEnum {\n    fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 432,
                            end: 434,
                            as_str(): "C0",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Boolean,
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 436,
                            end: 440,
                            as_str(): "bool",
                        },
                    ),
                    value: Expression {
                        kind: Literal(
                            Boolean(
                                true,
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 443,
                            end: 447,
                            as_str(): "true",
                        },
                    },
                    visibility: Public,
                    is_configurable: true,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 432,
                        end: 434,
                        as_str(): "C0",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 413,
            end: 723,
            as_str(): "configurable {\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 453,
                            end: 455,
                            as_str(): "C1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 457,
                            end: 460,
                            as_str(): "u64",
                        },
                    ),
                    value: Expression {
                        kind: Literal(
                            Numeric(
                                42,
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 463,
                            end: 465,
                            as_str(): "42",
                        },
                    },
                    visibility: Public,
                    is_configurable: true,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 453,
                        end: 455,
                        as_str(): "C1",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 413,
            end: 723,
            as_str(): "configurable {\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 471,
                            end: 473,
                            as_str(): "C2",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: B256,
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 475,
                            end: 479,
                            as_str(): "b256",
                        },
                    ),
                    value: Expression {
                        kind: Literal(
                            B256(
                                [
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                    17,
                                ],
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 482,
                            end: 548,
                            as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                        },
                    },
                    visibility: Public,
                    is_configurable: true,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 471,
                        end: 473,
                        as_str(): "C2",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 413,
            end: 723,
            as_str(): "configurable {\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 554,
                            end: 556,
                            as_str(): "C3",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 558,
                                end: 566,
                                as_str(): "MyStruct",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 558,
                            end: 566,
                            as_str(): "MyStruct",
                        },
                    ),
                    value: Expression {
                        kind: Struct(
                            StructExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 569,
                                                end: 577,
                                                as_str(): "MyStruct",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 569,
                                        end: 577,
                                        as_str(): "MyStruct",
                                    },
                                },
                                fields: [
                                    StructExpressionField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 580,
                                                end: 581,
                                                as_str(): "x",
                                            },
                                            is_raw_ident: false,
                                        },
                                        value: Expression {
                                            kind: Literal(
                                                Numeric(
                                                    42,
                                                ),
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 583,
                                                end: 585,
                                                as_str(): "42",
                                            },
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 580,
                                            end: 585,
                                            as_str(): "x: 42",
                                        },
                                    },
                                    StructExpressionField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 587,
                                                end: 588,
                                                as_str(): "y",
                                            },
                                            is_raw_ident: false,
                                        },
                                        value: Expression {
                                            kind: Literal(
                                                Boolean(
                                                    true,
                                                ),
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 590,
                                                end: 594,
                                                as_str(): "true",
                                            },
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 587,
                                            end: 594,
                                            as_str(): "y: true",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 569,
                            end: 596,
                            as_str(): "MyStruct { x: 42, y: true }",
                        },
                    },
                    visibility: Public,
                    is_configurable: true,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 554,
                        end: 556,
                        as_str(): "C3",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 413,
            end: 723,
            as_str(): "configurable {\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 602,
                            end: 604,
                            as_str(): "C4",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 606,
                                end: 612,
                                as_str(): "MyEnum",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 606,
                            end: 612,
                            as_str(): "MyEnum",
                        },
                    ),
                    value: Expression {
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
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 615,
                                                        end: 621,
                                                        as_str(): "MyEnum",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 615,
                                                    end: 621,
                                                    as_str(): "MyEnum",
                                                },
                                            },
                                            suffix: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 623,
                                                    end: 624,
                                                    as_str(): "A",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 615,
                                        end: 624,
                                        as_str(): "MyEnum::A",
                                    },
                                },
                                args: [
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                42,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 625,
                                            end: 627,
                                            as_str(): "42",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 615,
                            end: 628,
                            as_str(): "MyEnum::A(42)",
                        },
                    },
                    visibility: Public,
                    is_configurable: true,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 602,
                        end: 604,
                        as_str(): "C4",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 413,
            end: 723,
            as_str(): "configurable {\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 634,
                            end: 636,
                            as_str(): "C5",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 638,
                                end: 644,
                                as_str(): "MyEnum",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 638,
                            end: 644,
                            as_str(): "MyEnum",
                        },
                    ),
                    value: Expression {
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
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 647,
                                                        end: 653,
                                                        as_str(): "MyEnum",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 647,
                                                    end: 653,
                                                    as_str(): "MyEnum",
                                                },
                                            },
                                            suffix: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 655,
                                                    end: 656,
                                                    as_str(): "B",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 647,
                                        end: 656,
                                        as_str(): "MyEnum::B",
                                    },
                                },
                                args: [
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 657,
                                            end: 661,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 647,
                            end: 662,
                            as_str(): "MyEnum::B(true)",
                        },
                    },
                    visibility: Public,
                    is_configurable: true,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 634,
                        end: 636,
                        as_str(): "C5",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 413,
            end: 723,
            as_str(): "configurable {\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 668,
                            end: 670,
                            as_str(): "C6",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Str(
                        Length {
                            val: 4,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 676,
                                end: 677,
                                as_str(): "4",
                            },
                        },
                    ),
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 672,
                            end: 678,
                            as_str(): "str[4]",
                        },
                    ),
                    value: Expression {
                        kind: Literal(
                            String(
                                Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 682,
                                    end: 686,
                                    as_str(): "fuel",
                                },
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 681,
                            end: 687,
                            as_str(): "\"fuel\"",
                        },
                    },
                    visibility: Public,
                    is_configurable: true,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 668,
                        end: 670,
                        as_str(): "C6",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 413,
            end: 723,
            as_str(): "configurable {\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 693,
                            end: 695,
                            as_str(): "C7",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Array(
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 698,
                                end: 701,
                                as_str(): "u64",
                            },
                        },
                        Length {
                            val: 4,
                            span: Span {
                                src (ptr): 0x00007fb11f3db040,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                ),
                                start: 703,
                                end: 704,
                                as_str(): "4",
                            },
                        },
                    ),
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 697,
                            end: 705,
                            as_str(): "[u64; 4]",
                        },
                    ),
                    value: Expression {
                        kind: Array(
                            ArrayExpression {
                                contents: [
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 709,
                                            end: 710,
                                            as_str(): "1",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                2,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 712,
                                            end: 713,
                                            as_str(): "2",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                3,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 715,
                                            end: 716,
                                            as_str(): "3",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                4,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 718,
                                            end: 719,
                                            as_str(): "4",
                                        },
                                    },
                                ],
                                length_span: None,
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 708,
                            end: 720,
                            as_str(): "[1, 2, 3, 4]",
                        },
                    },
                    visibility: Public,
                    is_configurable: true,
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 693,
                        end: 695,
                        as_str(): "C7",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 413,
            end: 723,
            as_str(): "configurable {\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {
                        Inline: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 727,
                                        end: 733,
                                        as_str(): "inline",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 734,
                                            end: 739,
                                            as_str(): "never",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 725,
                                    end: 741,
                                    as_str(): "#[inline(never)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 745,
                            end: 759,
                            as_str(): "test_first_use",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 768,
                                                                end: 774,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 768,
                                                        end: 774,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 778,
                                                                                        end: 780,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 778,
                                                                                        end: 780,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 778,
                                                                                    end: 780,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 778,
                                                                        end: 780,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 775,
                                                                                    end: 777,
                                                                                    as_str(): "C0",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 775,
                                                                            end: 777,
                                                                            as_str(): "C0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 781,
                                                                            end: 785,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 775,
                                                            end: 785,
                                                            as_str(): "C0 == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 768,
                                            end: 786,
                                            as_str(): "assert(C0 == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 768,
                                    end: 786,
                                    as_str(): "assert(C0 == true)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 792,
                                                                end: 798,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 792,
                                                        end: 798,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 802,
                                                                                        end: 804,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 802,
                                                                                        end: 804,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 802,
                                                                                    end: 804,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 802,
                                                                        end: 804,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 799,
                                                                                    end: 801,
                                                                                    as_str(): "C1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 799,
                                                                            end: 801,
                                                                            as_str(): "C1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 805,
                                                                            end: 807,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 799,
                                                            end: 807,
                                                            as_str(): "C1 == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 792,
                                            end: 808,
                                            as_str(): "assert(C1 == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 792,
                                    end: 808,
                                    as_str(): "assert(C1 == 42)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 814,
                                                                end: 820,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 814,
                                                        end: 820,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 824,
                                                                                        end: 826,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 824,
                                                                                        end: 826,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 824,
                                                                                    end: 826,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 824,
                                                                        end: 826,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 821,
                                                                                    end: 823,
                                                                                    as_str(): "C2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 821,
                                                                            end: 823,
                                                                            as_str(): "C2",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            B256(
                                                                                [
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                ],
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 827,
                                                                            end: 893,
                                                                            as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 821,
                                                            end: 893,
                                                            as_str(): "C2 == 0x1111111111111111111111111111111111111111111111111111111111111111",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 814,
                                            end: 894,
                                            as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 814,
                                    end: 894,
                                    as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 900,
                                                                end: 906,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 900,
                                                        end: 906,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 912,
                                                                                        end: 914,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 912,
                                                                                        end: 914,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 912,
                                                                                    end: 914,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 912,
                                                                        end: 914,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 907,
                                                                                                end: 909,
                                                                                                as_str(): "C3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 907,
                                                                                        end: 909,
                                                                                        as_str(): "C3",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 910,
                                                                                        end: 911,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 907,
                                                                            end: 911,
                                                                            as_str(): "C3.x",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 915,
                                                                            end: 917,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 907,
                                                            end: 917,
                                                            as_str(): "C3.x == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 900,
                                            end: 918,
                                            as_str(): "assert(C3.x == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 900,
                                    end: 918,
                                    as_str(): "assert(C3.x == 42)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 924,
                                                                end: 930,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 924,
                                                        end: 930,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 936,
                                                                                        end: 938,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 936,
                                                                                        end: 938,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 936,
                                                                                    end: 938,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 936,
                                                                        end: 938,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 931,
                                                                                                end: 933,
                                                                                                as_str(): "C3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 931,
                                                                                        end: 933,
                                                                                        as_str(): "C3",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 934,
                                                                                        end: 935,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 931,
                                                                            end: 935,
                                                                            as_str(): "C3.y",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 939,
                                                                            end: 943,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 931,
                                                            end: 943,
                                                            as_str(): "C3.y == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 924,
                                            end: 944,
                                            as_str(): "assert(C3.y == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 924,
                                    end: 944,
                                    as_str(): "assert(C3.y == true)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 950,
                                                                end: 956,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 950,
                                                        end: 956,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 960,
                                                                                        end: 962,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 960,
                                                                                        end: 962,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 960,
                                                                                    end: 962,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 960,
                                                                        end: 962,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 957,
                                                                                    end: 959,
                                                                                    as_str(): "C4",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 957,
                                                                            end: 959,
                                                                            as_str(): "C4",
                                                                        },
                                                                    },
                                                                    Expression {
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 963,
                                                                                                        end: 969,
                                                                                                        as_str(): "MyEnum",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 963,
                                                                                                    end: 969,
                                                                                                    as_str(): "MyEnum",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 971,
                                                                                                    end: 972,
                                                                                                    as_str(): "A",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 963,
                                                                                        end: 972,
                                                                                        as_str(): "MyEnum::A",
                                                                                    },
                                                                                },
                                                                                args: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                42,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 973,
                                                                                            end: 975,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 963,
                                                                            end: 976,
                                                                            as_str(): "MyEnum::A(42)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 957,
                                                            end: 976,
                                                            as_str(): "C4 == MyEnum::A(42)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 950,
                                            end: 977,
                                            as_str(): "assert(C4 == MyEnum::A(42))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 950,
                                    end: 977,
                                    as_str(): "assert(C4 == MyEnum::A(42))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 983,
                                                                end: 989,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 983,
                                                        end: 989,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 993,
                                                                                        end: 995,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 993,
                                                                                        end: 995,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 993,
                                                                                    end: 995,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 993,
                                                                        end: 995,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 990,
                                                                                    end: 992,
                                                                                    as_str(): "C5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 990,
                                                                            end: 992,
                                                                            as_str(): "C5",
                                                                        },
                                                                    },
                                                                    Expression {
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 996,
                                                                                                        end: 1002,
                                                                                                        as_str(): "MyEnum",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 996,
                                                                                                    end: 1002,
                                                                                                    as_str(): "MyEnum",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1004,
                                                                                                    end: 1005,
                                                                                                    as_str(): "B",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 996,
                                                                                        end: 1005,
                                                                                        as_str(): "MyEnum::B",
                                                                                    },
                                                                                },
                                                                                args: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                true,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1006,
                                                                                            end: 1010,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 996,
                                                                            end: 1011,
                                                                            as_str(): "MyEnum::B(true)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 990,
                                                            end: 1011,
                                                            as_str(): "C5 == MyEnum::B(true)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 983,
                                            end: 1012,
                                            as_str(): "assert(C5 == MyEnum::B(true))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 983,
                                    end: 1012,
                                    as_str(): "assert(C5 == MyEnum::B(true))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1018,
                                                                end: 1024,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1018,
                                                        end: 1024,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1036,
                                                                                        end: 1038,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1036,
                                                                                        end: 1038,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1036,
                                                                                    end: 1038,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1036,
                                                                        end: 1038,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1025,
                                                                                                end: 1031,
                                                                                                as_str(): "sha256",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1025,
                                                                                        end: 1031,
                                                                                        as_str(): "sha256",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1032,
                                                                                                    end: 1034,
                                                                                                    as_str(): "C6",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1032,
                                                                                            end: 1034,
                                                                                            as_str(): "C6",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1025,
                                                                            end: 1035,
                                                                            as_str(): "sha256(C6)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1039,
                                                                                                end: 1045,
                                                                                                as_str(): "sha256",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1039,
                                                                                        end: 1045,
                                                                                        as_str(): "sha256",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            String(
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1047,
                                                                                                    end: 1051,
                                                                                                    as_str(): "fuel",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1046,
                                                                                            end: 1052,
                                                                                            as_str(): "\"fuel\"",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1039,
                                                                            end: 1053,
                                                                            as_str(): "sha256(\"fuel\")",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1025,
                                                            end: 1053,
                                                            as_str(): "sha256(C6) == sha256(\"fuel\")",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1018,
                                            end: 1054,
                                            as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1018,
                                    end: 1054,
                                    as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1060,
                                                                end: 1066,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1060,
                                                        end: 1066,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1073,
                                                                                        end: 1075,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1073,
                                                                                        end: 1075,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1073,
                                                                                    end: 1075,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1073,
                                                                        end: 1075,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1067,
                                                                                                end: 1069,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1067,
                                                                                        end: 1069,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1070,
                                                                                        end: 1071,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1067,
                                                                            end: 1072,
                                                                            as_str(): "C7[0]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1076,
                                                                            end: 1077,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1067,
                                                            end: 1077,
                                                            as_str(): "C7[0] == 1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1060,
                                            end: 1078,
                                            as_str(): "assert(C7[0] == 1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1060,
                                    end: 1078,
                                    as_str(): "assert(C7[0] == 1)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1084,
                                                                end: 1090,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1084,
                                                        end: 1090,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1097,
                                                                                        end: 1099,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1097,
                                                                                        end: 1099,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1097,
                                                                                    end: 1099,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1097,
                                                                        end: 1099,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1091,
                                                                                                end: 1093,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1091,
                                                                                        end: 1093,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            1,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1094,
                                                                                        end: 1095,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1091,
                                                                            end: 1096,
                                                                            as_str(): "C7[1]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1100,
                                                                            end: 1101,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1091,
                                                            end: 1101,
                                                            as_str(): "C7[1] == 2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1084,
                                            end: 1102,
                                            as_str(): "assert(C7[1] == 2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1084,
                                    end: 1102,
                                    as_str(): "assert(C7[1] == 2)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1108,
                                                                end: 1114,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1108,
                                                        end: 1114,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1121,
                                                                                        end: 1123,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1121,
                                                                                        end: 1123,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1121,
                                                                                    end: 1123,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1121,
                                                                        end: 1123,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1115,
                                                                                                end: 1117,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1115,
                                                                                        end: 1117,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            2,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1118,
                                                                                        end: 1119,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1115,
                                                                            end: 1120,
                                                                            as_str(): "C7[2]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1124,
                                                                            end: 1125,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1115,
                                                            end: 1125,
                                                            as_str(): "C7[2] == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1108,
                                            end: 1126,
                                            as_str(): "assert(C7[2] == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1108,
                                    end: 1126,
                                    as_str(): "assert(C7[2] == 3)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1132,
                                                                end: 1138,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1132,
                                                        end: 1138,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1145,
                                                                                        end: 1147,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1145,
                                                                                        end: 1147,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1145,
                                                                                    end: 1147,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1145,
                                                                        end: 1147,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1139,
                                                                                                end: 1141,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1139,
                                                                                        end: 1141,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            3,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1142,
                                                                                        end: 1143,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1139,
                                                                            end: 1144,
                                                                            as_str(): "C7[3]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1148,
                                                                            end: 1149,
                                                                            as_str(): "4",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1139,
                                                            end: 1149,
                                                            as_str(): "C7[3] == 4",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1132,
                                            end: 1150,
                                            as_str(): "assert(C7[3] == 4)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1132,
                                    end: 1150,
                                    as_str(): "assert(C7[3] == 4)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 762,
                            end: 1153,
                            as_str(): "{\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 742,
                        end: 1153,
                        as_str(): "fn test_first_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 742,
                        end: 761,
                        as_str(): "fn test_first_use()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 725,
            end: 1153,
            as_str(): "#[inline(never)]\nfn test_first_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {
                        Inline: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1157,
                                        end: 1163,
                                        as_str(): "inline",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1164,
                                            end: 1169,
                                            as_str(): "never",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1155,
                                    end: 1171,
                                    as_str(): "#[inline(never)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1175,
                            end: 1190,
                            as_str(): "test_second_use",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1199,
                                                                end: 1205,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1199,
                                                        end: 1205,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1209,
                                                                                        end: 1211,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1209,
                                                                                        end: 1211,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1209,
                                                                                    end: 1211,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1209,
                                                                        end: 1211,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1206,
                                                                                    end: 1208,
                                                                                    as_str(): "C0",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1206,
                                                                            end: 1208,
                                                                            as_str(): "C0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1212,
                                                                            end: 1216,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1206,
                                                            end: 1216,
                                                            as_str(): "C0 == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1199,
                                            end: 1217,
                                            as_str(): "assert(C0 == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1199,
                                    end: 1217,
                                    as_str(): "assert(C0 == true)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1223,
                                                                end: 1229,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1223,
                                                        end: 1229,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1233,
                                                                                        end: 1235,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1233,
                                                                                        end: 1235,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1233,
                                                                                    end: 1235,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1233,
                                                                        end: 1235,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1230,
                                                                                    end: 1232,
                                                                                    as_str(): "C1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1230,
                                                                            end: 1232,
                                                                            as_str(): "C1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1236,
                                                                            end: 1238,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1230,
                                                            end: 1238,
                                                            as_str(): "C1 == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1223,
                                            end: 1239,
                                            as_str(): "assert(C1 == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1223,
                                    end: 1239,
                                    as_str(): "assert(C1 == 42)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1245,
                                                                end: 1251,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1245,
                                                        end: 1251,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1255,
                                                                                        end: 1257,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1255,
                                                                                        end: 1257,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1255,
                                                                                    end: 1257,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1255,
                                                                        end: 1257,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1252,
                                                                                    end: 1254,
                                                                                    as_str(): "C2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1252,
                                                                            end: 1254,
                                                                            as_str(): "C2",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            B256(
                                                                                [
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                ],
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1258,
                                                                            end: 1324,
                                                                            as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1252,
                                                            end: 1324,
                                                            as_str(): "C2 == 0x1111111111111111111111111111111111111111111111111111111111111111",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1245,
                                            end: 1325,
                                            as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1245,
                                    end: 1325,
                                    as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1331,
                                                                end: 1337,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1331,
                                                        end: 1337,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1343,
                                                                                        end: 1345,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1343,
                                                                                        end: 1345,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1343,
                                                                                    end: 1345,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1343,
                                                                        end: 1345,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1338,
                                                                                                end: 1340,
                                                                                                as_str(): "C3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1338,
                                                                                        end: 1340,
                                                                                        as_str(): "C3",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1341,
                                                                                        end: 1342,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1338,
                                                                            end: 1342,
                                                                            as_str(): "C3.x",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1346,
                                                                            end: 1348,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1338,
                                                            end: 1348,
                                                            as_str(): "C3.x == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1331,
                                            end: 1349,
                                            as_str(): "assert(C3.x == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1331,
                                    end: 1349,
                                    as_str(): "assert(C3.x == 42)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1355,
                                                                end: 1361,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1355,
                                                        end: 1361,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1367,
                                                                                        end: 1369,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1367,
                                                                                        end: 1369,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1367,
                                                                                    end: 1369,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1367,
                                                                        end: 1369,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1362,
                                                                                                end: 1364,
                                                                                                as_str(): "C3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1362,
                                                                                        end: 1364,
                                                                                        as_str(): "C3",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1365,
                                                                                        end: 1366,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1362,
                                                                            end: 1366,
                                                                            as_str(): "C3.y",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1370,
                                                                            end: 1374,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1362,
                                                            end: 1374,
                                                            as_str(): "C3.y == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1355,
                                            end: 1375,
                                            as_str(): "assert(C3.y == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1355,
                                    end: 1375,
                                    as_str(): "assert(C3.y == true)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1381,
                                                                end: 1387,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1381,
                                                        end: 1387,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1391,
                                                                                        end: 1393,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1391,
                                                                                        end: 1393,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1391,
                                                                                    end: 1393,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1391,
                                                                        end: 1393,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1388,
                                                                                    end: 1390,
                                                                                    as_str(): "C4",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1388,
                                                                            end: 1390,
                                                                            as_str(): "C4",
                                                                        },
                                                                    },
                                                                    Expression {
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1394,
                                                                                                        end: 1400,
                                                                                                        as_str(): "MyEnum",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1394,
                                                                                                    end: 1400,
                                                                                                    as_str(): "MyEnum",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1402,
                                                                                                    end: 1403,
                                                                                                    as_str(): "A",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1394,
                                                                                        end: 1403,
                                                                                        as_str(): "MyEnum::A",
                                                                                    },
                                                                                },
                                                                                args: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                42,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1404,
                                                                                            end: 1406,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1394,
                                                                            end: 1407,
                                                                            as_str(): "MyEnum::A(42)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1388,
                                                            end: 1407,
                                                            as_str(): "C4 == MyEnum::A(42)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1381,
                                            end: 1408,
                                            as_str(): "assert(C4 == MyEnum::A(42))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1381,
                                    end: 1408,
                                    as_str(): "assert(C4 == MyEnum::A(42))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1414,
                                                                end: 1420,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1414,
                                                        end: 1420,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1424,
                                                                                        end: 1426,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1424,
                                                                                        end: 1426,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1424,
                                                                                    end: 1426,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1424,
                                                                        end: 1426,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1421,
                                                                                    end: 1423,
                                                                                    as_str(): "C5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1421,
                                                                            end: 1423,
                                                                            as_str(): "C5",
                                                                        },
                                                                    },
                                                                    Expression {
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1427,
                                                                                                        end: 1433,
                                                                                                        as_str(): "MyEnum",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1427,
                                                                                                    end: 1433,
                                                                                                    as_str(): "MyEnum",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1435,
                                                                                                    end: 1436,
                                                                                                    as_str(): "B",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1427,
                                                                                        end: 1436,
                                                                                        as_str(): "MyEnum::B",
                                                                                    },
                                                                                },
                                                                                args: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                true,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1437,
                                                                                            end: 1441,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1427,
                                                                            end: 1442,
                                                                            as_str(): "MyEnum::B(true)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1421,
                                                            end: 1442,
                                                            as_str(): "C5 == MyEnum::B(true)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1414,
                                            end: 1443,
                                            as_str(): "assert(C5 == MyEnum::B(true))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1414,
                                    end: 1443,
                                    as_str(): "assert(C5 == MyEnum::B(true))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1449,
                                                                end: 1455,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1449,
                                                        end: 1455,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1467,
                                                                                        end: 1469,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1467,
                                                                                        end: 1469,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1467,
                                                                                    end: 1469,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1467,
                                                                        end: 1469,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1456,
                                                                                                end: 1462,
                                                                                                as_str(): "sha256",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1456,
                                                                                        end: 1462,
                                                                                        as_str(): "sha256",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1463,
                                                                                                    end: 1465,
                                                                                                    as_str(): "C6",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1463,
                                                                                            end: 1465,
                                                                                            as_str(): "C6",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1456,
                                                                            end: 1466,
                                                                            as_str(): "sha256(C6)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1470,
                                                                                                end: 1476,
                                                                                                as_str(): "sha256",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1470,
                                                                                        end: 1476,
                                                                                        as_str(): "sha256",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            String(
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1478,
                                                                                                    end: 1482,
                                                                                                    as_str(): "fuel",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1477,
                                                                                            end: 1483,
                                                                                            as_str(): "\"fuel\"",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1470,
                                                                            end: 1484,
                                                                            as_str(): "sha256(\"fuel\")",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1456,
                                                            end: 1484,
                                                            as_str(): "sha256(C6) == sha256(\"fuel\")",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1449,
                                            end: 1485,
                                            as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1449,
                                    end: 1485,
                                    as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1491,
                                                                end: 1497,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1491,
                                                        end: 1497,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1504,
                                                                                        end: 1506,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1504,
                                                                                        end: 1506,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1504,
                                                                                    end: 1506,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1504,
                                                                        end: 1506,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1498,
                                                                                                end: 1500,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1498,
                                                                                        end: 1500,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1501,
                                                                                        end: 1502,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1498,
                                                                            end: 1503,
                                                                            as_str(): "C7[0]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1507,
                                                                            end: 1508,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1498,
                                                            end: 1508,
                                                            as_str(): "C7[0] == 1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1491,
                                            end: 1509,
                                            as_str(): "assert(C7[0] == 1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1491,
                                    end: 1509,
                                    as_str(): "assert(C7[0] == 1)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1515,
                                                                end: 1521,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1515,
                                                        end: 1521,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1528,
                                                                                        end: 1530,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1528,
                                                                                        end: 1530,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1528,
                                                                                    end: 1530,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1528,
                                                                        end: 1530,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1522,
                                                                                                end: 1524,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1522,
                                                                                        end: 1524,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            1,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1525,
                                                                                        end: 1526,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1522,
                                                                            end: 1527,
                                                                            as_str(): "C7[1]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1531,
                                                                            end: 1532,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1522,
                                                            end: 1532,
                                                            as_str(): "C7[1] == 2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1515,
                                            end: 1533,
                                            as_str(): "assert(C7[1] == 2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1515,
                                    end: 1533,
                                    as_str(): "assert(C7[1] == 2)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1539,
                                                                end: 1545,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1539,
                                                        end: 1545,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1552,
                                                                                        end: 1554,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1552,
                                                                                        end: 1554,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1552,
                                                                                    end: 1554,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1552,
                                                                        end: 1554,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1546,
                                                                                                end: 1548,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1546,
                                                                                        end: 1548,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            2,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1549,
                                                                                        end: 1550,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1546,
                                                                            end: 1551,
                                                                            as_str(): "C7[2]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1555,
                                                                            end: 1556,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1546,
                                                            end: 1556,
                                                            as_str(): "C7[2] == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1539,
                                            end: 1557,
                                            as_str(): "assert(C7[2] == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1539,
                                    end: 1557,
                                    as_str(): "assert(C7[2] == 3)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1563,
                                                                end: 1569,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1563,
                                                        end: 1569,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1576,
                                                                                        end: 1578,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1576,
                                                                                        end: 1578,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1576,
                                                                                    end: 1578,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1576,
                                                                        end: 1578,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1570,
                                                                                                end: 1572,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1570,
                                                                                        end: 1572,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            3,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1573,
                                                                                        end: 1574,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1570,
                                                                            end: 1575,
                                                                            as_str(): "C7[3]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1579,
                                                                            end: 1580,
                                                                            as_str(): "4",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1570,
                                                            end: 1580,
                                                            as_str(): "C7[3] == 4",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1563,
                                            end: 1581,
                                            as_str(): "assert(C7[3] == 4)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1563,
                                    end: 1581,
                                    as_str(): "assert(C7[3] == 4)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1193,
                            end: 1584,
                            as_str(): "{\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 1172,
                        end: 1584,
                        as_str(): "fn test_second_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 1172,
                        end: 1192,
                        as_str(): "fn test_second_use()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 1155,
            end: 1584,
            as_str(): "#[inline(never)]\nfn test_second_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {
                        Inline: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1588,
                                        end: 1594,
                                        as_str(): "inline",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1595,
                                            end: 1601,
                                            as_str(): "always",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1586,
                                    end: 1603,
                                    as_str(): "#[inline(always)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1607,
                            end: 1622,
                            as_str(): "test_inline_use",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1631,
                                                                end: 1637,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1631,
                                                        end: 1637,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1641,
                                                                                        end: 1643,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1641,
                                                                                        end: 1643,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1641,
                                                                                    end: 1643,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1641,
                                                                        end: 1643,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1638,
                                                                                    end: 1640,
                                                                                    as_str(): "C0",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1638,
                                                                            end: 1640,
                                                                            as_str(): "C0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1644,
                                                                            end: 1648,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1638,
                                                            end: 1648,
                                                            as_str(): "C0 == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1631,
                                            end: 1649,
                                            as_str(): "assert(C0 == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1631,
                                    end: 1649,
                                    as_str(): "assert(C0 == true)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1655,
                                                                end: 1661,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1655,
                                                        end: 1661,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1665,
                                                                                        end: 1667,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1665,
                                                                                        end: 1667,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1665,
                                                                                    end: 1667,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1665,
                                                                        end: 1667,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1662,
                                                                                    end: 1664,
                                                                                    as_str(): "C1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1662,
                                                                            end: 1664,
                                                                            as_str(): "C1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1668,
                                                                            end: 1670,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1662,
                                                            end: 1670,
                                                            as_str(): "C1 == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1655,
                                            end: 1671,
                                            as_str(): "assert(C1 == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1655,
                                    end: 1671,
                                    as_str(): "assert(C1 == 42)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1677,
                                                                end: 1683,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1677,
                                                        end: 1683,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1687,
                                                                                        end: 1689,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1687,
                                                                                        end: 1689,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1687,
                                                                                    end: 1689,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1687,
                                                                        end: 1689,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1684,
                                                                                    end: 1686,
                                                                                    as_str(): "C2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1684,
                                                                            end: 1686,
                                                                            as_str(): "C2",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            B256(
                                                                                [
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                    17,
                                                                                ],
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1690,
                                                                            end: 1756,
                                                                            as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1684,
                                                            end: 1756,
                                                            as_str(): "C2 == 0x1111111111111111111111111111111111111111111111111111111111111111",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1677,
                                            end: 1757,
                                            as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1677,
                                    end: 1757,
                                    as_str(): "assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1763,
                                                                end: 1769,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1763,
                                                        end: 1769,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1775,
                                                                                        end: 1777,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1775,
                                                                                        end: 1777,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1775,
                                                                                    end: 1777,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1775,
                                                                        end: 1777,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1770,
                                                                                                end: 1772,
                                                                                                as_str(): "C3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1770,
                                                                                        end: 1772,
                                                                                        as_str(): "C3",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1773,
                                                                                        end: 1774,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1770,
                                                                            end: 1774,
                                                                            as_str(): "C3.x",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1778,
                                                                            end: 1780,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1770,
                                                            end: 1780,
                                                            as_str(): "C3.x == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1763,
                                            end: 1781,
                                            as_str(): "assert(C3.x == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1763,
                                    end: 1781,
                                    as_str(): "assert(C3.x == 42)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1787,
                                                                end: 1793,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1787,
                                                        end: 1793,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1799,
                                                                                        end: 1801,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1799,
                                                                                        end: 1801,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1799,
                                                                                    end: 1801,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1799,
                                                                        end: 1801,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1794,
                                                                                                end: 1796,
                                                                                                as_str(): "C3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1794,
                                                                                        end: 1796,
                                                                                        as_str(): "C3",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1797,
                                                                                        end: 1798,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1794,
                                                                            end: 1798,
                                                                            as_str(): "C3.y",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1802,
                                                                            end: 1806,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1794,
                                                            end: 1806,
                                                            as_str(): "C3.y == true",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1787,
                                            end: 1807,
                                            as_str(): "assert(C3.y == true)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1787,
                                    end: 1807,
                                    as_str(): "assert(C3.y == true)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1813,
                                                                end: 1819,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1813,
                                                        end: 1819,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1823,
                                                                                        end: 1825,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1823,
                                                                                        end: 1825,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1823,
                                                                                    end: 1825,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1823,
                                                                        end: 1825,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1820,
                                                                                    end: 1822,
                                                                                    as_str(): "C4",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1820,
                                                                            end: 1822,
                                                                            as_str(): "C4",
                                                                        },
                                                                    },
                                                                    Expression {
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1826,
                                                                                                        end: 1832,
                                                                                                        as_str(): "MyEnum",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1826,
                                                                                                    end: 1832,
                                                                                                    as_str(): "MyEnum",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1834,
                                                                                                    end: 1835,
                                                                                                    as_str(): "A",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1826,
                                                                                        end: 1835,
                                                                                        as_str(): "MyEnum::A",
                                                                                    },
                                                                                },
                                                                                args: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                42,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1836,
                                                                                            end: 1838,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1826,
                                                                            end: 1839,
                                                                            as_str(): "MyEnum::A(42)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1820,
                                                            end: 1839,
                                                            as_str(): "C4 == MyEnum::A(42)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1813,
                                            end: 1840,
                                            as_str(): "assert(C4 == MyEnum::A(42))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1813,
                                    end: 1840,
                                    as_str(): "assert(C4 == MyEnum::A(42))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1846,
                                                                end: 1852,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1846,
                                                        end: 1852,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1856,
                                                                                        end: 1858,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1856,
                                                                                        end: 1858,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1856,
                                                                                    end: 1858,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1856,
                                                                        end: 1858,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1853,
                                                                                    end: 1855,
                                                                                    as_str(): "C5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1853,
                                                                            end: 1855,
                                                                            as_str(): "C5",
                                                                        },
                                                                    },
                                                                    Expression {
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1859,
                                                                                                        end: 1865,
                                                                                                        as_str(): "MyEnum",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1859,
                                                                                                    end: 1865,
                                                                                                    as_str(): "MyEnum",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1867,
                                                                                                    end: 1868,
                                                                                                    as_str(): "B",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1859,
                                                                                        end: 1868,
                                                                                        as_str(): "MyEnum::B",
                                                                                    },
                                                                                },
                                                                                args: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                true,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1869,
                                                                                            end: 1873,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1859,
                                                                            end: 1874,
                                                                            as_str(): "MyEnum::B(true)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1853,
                                                            end: 1874,
                                                            as_str(): "C5 == MyEnum::B(true)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1846,
                                            end: 1875,
                                            as_str(): "assert(C5 == MyEnum::B(true))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1846,
                                    end: 1875,
                                    as_str(): "assert(C5 == MyEnum::B(true))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1881,
                                                                end: 1887,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1881,
                                                        end: 1887,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1899,
                                                                                        end: 1901,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1899,
                                                                                        end: 1901,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1899,
                                                                                    end: 1901,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1899,
                                                                        end: 1901,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1888,
                                                                                                end: 1894,
                                                                                                as_str(): "sha256",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1888,
                                                                                        end: 1894,
                                                                                        as_str(): "sha256",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1895,
                                                                                                    end: 1897,
                                                                                                    as_str(): "C6",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1895,
                                                                                            end: 1897,
                                                                                            as_str(): "C6",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1888,
                                                                            end: 1898,
                                                                            as_str(): "sha256(C6)",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1902,
                                                                                                end: 1908,
                                                                                                as_str(): "sha256",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1902,
                                                                                        end: 1908,
                                                                                        as_str(): "sha256",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            String(
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1910,
                                                                                                    end: 1914,
                                                                                                    as_str(): "fuel",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1909,
                                                                                            end: 1915,
                                                                                            as_str(): "\"fuel\"",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1902,
                                                                            end: 1916,
                                                                            as_str(): "sha256(\"fuel\")",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1888,
                                                            end: 1916,
                                                            as_str(): "sha256(C6) == sha256(\"fuel\")",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1881,
                                            end: 1917,
                                            as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1881,
                                    end: 1917,
                                    as_str(): "assert(sha256(C6) == sha256(\"fuel\"))",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1923,
                                                                end: 1929,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1923,
                                                        end: 1929,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1936,
                                                                                        end: 1938,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1936,
                                                                                        end: 1938,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1936,
                                                                                    end: 1938,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1936,
                                                                        end: 1938,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1930,
                                                                                                end: 1932,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1930,
                                                                                        end: 1932,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            0,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1933,
                                                                                        end: 1934,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1930,
                                                                            end: 1935,
                                                                            as_str(): "C7[0]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1939,
                                                                            end: 1940,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1930,
                                                            end: 1940,
                                                            as_str(): "C7[0] == 1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1923,
                                            end: 1941,
                                            as_str(): "assert(C7[0] == 1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1923,
                                    end: 1941,
                                    as_str(): "assert(C7[0] == 1)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1947,
                                                                end: 1953,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1947,
                                                        end: 1953,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1960,
                                                                                        end: 1962,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1960,
                                                                                        end: 1962,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1960,
                                                                                    end: 1962,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1960,
                                                                        end: 1962,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1954,
                                                                                                end: 1956,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1954,
                                                                                        end: 1956,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            1,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1957,
                                                                                        end: 1958,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1954,
                                                                            end: 1959,
                                                                            as_str(): "C7[1]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1963,
                                                                            end: 1964,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1954,
                                                            end: 1964,
                                                            as_str(): "C7[1] == 2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1947,
                                            end: 1965,
                                            as_str(): "assert(C7[1] == 2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1947,
                                    end: 1965,
                                    as_str(): "assert(C7[1] == 2)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1971,
                                                                end: 1977,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1971,
                                                        end: 1977,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1984,
                                                                                        end: 1986,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1984,
                                                                                        end: 1986,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1984,
                                                                                    end: 1986,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1984,
                                                                        end: 1986,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1978,
                                                                                                end: 1980,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1978,
                                                                                        end: 1980,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            2,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1981,
                                                                                        end: 1982,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1978,
                                                                            end: 1983,
                                                                            as_str(): "C7[2]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                3,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1987,
                                                                            end: 1988,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1978,
                                                            end: 1988,
                                                            as_str(): "C7[2] == 3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1971,
                                            end: 1989,
                                            as_str(): "assert(C7[2] == 3)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1971,
                                    end: 1989,
                                    as_str(): "assert(C7[2] == 3)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 1995,
                                                                end: 2001,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1995,
                                                        end: 2001,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 2008,
                                                                                        end: 2010,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 2008,
                                                                                        end: 2010,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 2008,
                                                                                    end: 2010,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 2008,
                                                                        end: 2010,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: ArrayIndex(
                                                                            ArrayIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 2002,
                                                                                                end: 2004,
                                                                                                as_str(): "C7",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 2002,
                                                                                        end: 2004,
                                                                                        as_str(): "C7",
                                                                                    },
                                                                                },
                                                                                index: Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            3,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 2005,
                                                                                        end: 2006,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 2002,
                                                                            end: 2007,
                                                                            as_str(): "C7[3]",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                4,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 2011,
                                                                            end: 2012,
                                                                            as_str(): "4",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2002,
                                                            end: 2012,
                                                            as_str(): "C7[3] == 4",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1995,
                                            end: 2013,
                                            as_str(): "assert(C7[3] == 4)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 1995,
                                    end: 2013,
                                    as_str(): "assert(C7[3] == 4)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 1625,
                            end: 2016,
                            as_str(): "{\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 1604,
                        end: 2016,
                        as_str(): "fn test_inline_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 1604,
                        end: 1624,
                        as_str(): "fn test_inline_use()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 1586,
            end: 2016,
            as_str(): "#[inline(always)]\nfn test_inline_use() {\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {
                        Inline: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2020,
                                        end: 2026,
                                        as_str(): "inline",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2027,
                                            end: 2032,
                                            as_str(): "never",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 2018,
                                    end: 2034,
                                    as_str(): "#[inline(never)]",
                                },
                            },
                        ],
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2038,
                            end: 2055,
                            as_str(): "test_various_uses",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 2064,
                                                                end: 2078,
                                                                as_str(): "test_first_use",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 2064,
                                                        end: 2078,
                                                        as_str(): "test_first_use",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2064,
                                            end: 2080,
                                            as_str(): "test_first_use()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 2064,
                                    end: 2080,
                                    as_str(): "test_first_use()",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 2086,
                                                                end: 2101,
                                                                as_str(): "test_second_use",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 2086,
                                                        end: 2101,
                                                        as_str(): "test_second_use",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2086,
                                            end: 2103,
                                            as_str(): "test_second_use()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 2086,
                                    end: 2103,
                                    as_str(): "test_second_use()",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 2109,
                                                                end: 2124,
                                                                as_str(): "test_inline_use",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 2109,
                                                        end: 2124,
                                                        as_str(): "test_inline_use",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2109,
                                            end: 2126,
                                            as_str(): "test_inline_use()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 2109,
                                    end: 2126,
                                    as_str(): "test_inline_use()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2058,
                            end: 2129,
                            as_str(): "{\n    test_first_use();\n    test_second_use();\n    test_inline_use();\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 2035,
                        end: 2129,
                        as_str(): "fn test_various_uses() {\n    test_first_use();\n    test_second_use();\n    test_inline_use();\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 2035,
                        end: 2057,
                        as_str(): "fn test_various_uses()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 2018,
            end: 2129,
            as_str(): "#[inline(never)]\nfn test_various_uses() {\n    test_first_use();\n    test_second_use();\n    test_inline_use();\n}",
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
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2134,
                            end: 2138,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 2147,
                                                                end: 2164,
                                                                as_str(): "test_various_uses",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 2147,
                                                        end: 2164,
                                                        as_str(): "test_various_uses",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2147,
                                            end: 2166,
                                            as_str(): "test_various_uses()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb11f3db040,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                    ),
                                    start: 2147,
                                    end: 2166,
                                    as_str(): "test_various_uses()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 2141,
                            end: 2169,
                            as_str(): "{\n    test_various_uses();\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 2131,
                        end: 2169,
                        as_str(): "fn main() {\n    test_various_uses();\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                        ),
                        start: 2131,
                        end: 2140,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb11f3db040,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
            ),
            start: 2131,
            end: 2169,
            as_str(): "fn main() {\n    test_various_uses();\n}",
        },
    },
]
