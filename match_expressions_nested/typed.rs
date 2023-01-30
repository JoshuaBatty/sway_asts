TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09612d790,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
            ),
            start: 43,
            end: 47,
            as_str(): "Zoom",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09612d790,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                    ),
                    start: 54,
                    end: 57,
                    as_str(): "Wow",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                33,
            ),
            initial_type_id: TypeId(
                33,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09612d790,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                ),
                start: 59,
                end: 62,
                as_str(): "u32",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe09612d790,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                ),
                start: 54,
                end: 62,
                as_str(): "Wow: u32",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe09612d790,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
        ),
        start: 38,
        end: 65,
        as_str(): "enum Zoom {\n    Wow: u32,\n}",
    },
    visibility: Private,
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09612d790,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
            ),
            start: 14,
            end: 17,
            as_str(): "Foo",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe09612d790,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                    ),
                    start: 24,
                    end: 27,
                    as_str(): "Bar",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31631,
            ),
            initial_type_id: TypeId(
                31630,
            ),
            type_span: Span {
                src (ptr): 0x00007fe09612d790,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                ),
                start: 29,
                end: 33,
                as_str(): "Zoom",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe09612d790,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                ),
                start: 24,
                end: 33,
                as_str(): "Bar: Zoom",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe09612d790,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
        ),
        start: 9,
        end: 36,
        as_str(): "enum Foo {\n    Bar: Zoom,\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09612d790,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
            ),
            start: 70,
            end: 74,
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
                                    src (ptr): 0x00007fe09612d790,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 95,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe09612d790,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                ),
                                                start: 14,
                                                end: 17,
                                                as_str(): "Foo",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe09612d790,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                        ),
                                                        start: 24,
                                                        end: 27,
                                                        as_str(): "Bar",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    31631,
                                                ),
                                                initial_type_id: TypeId(
                                                    31630,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 29,
                                                    end: 33,
                                                    as_str(): "Zoom",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 33,
                                                    as_str(): "Bar: Zoom",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 36,
                                            as_str(): "enum Foo {\n    Bar: Zoom,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 24,
                                            end: 27,
                                            as_str(): "Bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: EnumInstantiation {
                                                enum_decl: TyEnumDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 43,
                                                            end: 47,
                                                            as_str(): "Zoom",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    type_parameters: [],
                                                    attributes: {},
                                                    variants: [
                                                        TyEnumVariant {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09612d790,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                    ),
                                                                    start: 54,
                                                                    end: 57,
                                                                    as_str(): "Wow",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            type_id: TypeId(
                                                                33,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                33,
                                                            ),
                                                            type_span: Span {
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 59,
                                                                end: 62,
                                                                as_str(): "u32",
                                                            },
                                                            tag: 0,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 54,
                                                                end: 62,
                                                                as_str(): "Wow: u32",
                                                            },
                                                            attributes: {},
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09612d790,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                        ),
                                                        start: 38,
                                                        end: 65,
                                                        as_str(): "enum Zoom {\n    Wow: u32,\n}",
                                                    },
                                                    visibility: Private,
                                                },
                                                variant_name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe09612d790,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                        ),
                                                        start: 54,
                                                        end: 57,
                                                        as_str(): "Wow",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                tag: 0,
                                                contents: Some(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                123,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 117,
                                                            end: 120,
                                                            as_str(): "123",
                                                        },
                                                    },
                                                ),
                                                enum_instantiation_span: Span {
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 107,
                                                    end: 111,
                                                    as_str(): "Zoom",
                                                },
                                                variant_instantiation_span: Span {
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 116,
                                                    as_str(): "Wow",
                                                },
                                                type_binding: TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09612d790,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                        ),
                                                        start: 107,
                                                        end: 116,
                                                        as_str(): "Zoom::Wow",
                                                    },
                                                },
                                            },
                                            return_type: TypeId(
                                                31631,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe09612d790,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                ),
                                                start: 113,
                                                end: 116,
                                                as_str(): "Wow",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 98,
                                        end: 101,
                                        as_str(): "Foo",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 106,
                                        as_str(): "Bar",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 106,
                                            as_str(): "Foo::Bar",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31634,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09612d790,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                    ),
                                    start: 103,
                                    end: 106,
                                    as_str(): "Bar",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31634,
                            ),
                            type_ascription: TypeId(
                                31633,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09612d790,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                    ),
                    start: 90,
                    end: 123,
                    as_str(): "let x = Foo::Bar(Zoom::Wow(123));",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
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
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 135,
                                                            as_str(): "x",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09612d790,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                    ),
                                                                    start: 94,
                                                                    end: 95,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 135,
                                                                as_str(): "x",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31634,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 135,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31634,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31638,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 180,
                                            as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                                        },
                                    },
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
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
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 135,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 135,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 134,
                                                                                end: 135,
                                                                                as_str(): "x",
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
                                                                                    src (ptr): 0x00007fe09eae6480,
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
                                                                                expression: EnumTag {
                                                                                    exp: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "__match_return_var_name_1",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 134,
                                                                                                    end: 135,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 134,
                                                                                                end: 135,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31634,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 134,
                                                                                            end: 135,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 135,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09eae6480,
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
                                                                                    U64(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 135,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13319,
                                                                        Span {
                                                                            src (ptr): 0x00007fe09eae6480,
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
                                                                    src (ptr): 0x00007fe09612d790,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                    ),
                                                                    start: 134,
                                                                    end: 135,
                                                                    as_str(): "x",
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
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 146,
                                                                                    end: 168,
                                                                                    as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 146,
                                                                                    end: 168,
                                                                                    as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 146,
                                                                                end: 168,
                                                                                as_str(): "Foo::Bar(Zoom::Wow(x))",
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
                                                                                    src (ptr): 0x00007fe09eae6480,
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
                                                                                expression: EnumTag {
                                                                                    exp: TyExpression {
                                                                                        expression: UnsafeDowncast {
                                                                                            exp: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_1",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                            ),
                                                                                                            start: 134,
                                                                                                            end: 135,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 134,
                                                                                                        end: 135,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    31634,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 134,
                                                                                                    end: 135,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                            },
                                                                                            variant: TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 24,
                                                                                                        end: 27,
                                                                                                        as_str(): "Bar",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    31631,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    31630,
                                                                                                ),
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 29,
                                                                                                    end: 33,
                                                                                                    as_str(): "Zoom",
                                                                                                },
                                                                                                tag: 0,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 24,
                                                                                                    end: 33,
                                                                                                    as_str(): "Bar: Zoom",
                                                                                                },
                                                                                                attributes: {},
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31631,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 146,
                                                                                            end: 168,
                                                                                            as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 146,
                                                                                    end: 168,
                                                                                    as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09eae6480,
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
                                                                                    U64(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 146,
                                                                                    end: 168,
                                                                                    as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13318,
                                                                        Span {
                                                                            src (ptr): 0x00007fe09eae6480,
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
                                                                    src (ptr): 0x00007fe09612d790,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                    ),
                                                                    start: 146,
                                                                    end: 168,
                                                                    as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 168,
                                                            as_str(): "x {\n        Foo::Bar(Zoom::Wow(x))",
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
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 165,
                                                                                            end: 166,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    body: TyExpression {
                                                                                        expression: UnsafeDowncast {
                                                                                            exp: TyExpression {
                                                                                                expression: UnsafeDowncast {
                                                                                                    exp: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "__match_return_var_name_1",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 134,
                                                                                                                    end: 135,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                ),
                                                                                                                start: 134,
                                                                                                                end: 135,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            31634,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                            ),
                                                                                                            start: 134,
                                                                                                            end: 135,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                    },
                                                                                                    variant: TyEnumVariant {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                ),
                                                                                                                start: 24,
                                                                                                                end: 27,
                                                                                                                as_str(): "Bar",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        type_id: TypeId(
                                                                                                            31631,
                                                                                                        ),
                                                                                                        initial_type_id: TypeId(
                                                                                                            31630,
                                                                                                        ),
                                                                                                        type_span: Span {
                                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                            ),
                                                                                                            start: 29,
                                                                                                            end: 33,
                                                                                                            as_str(): "Zoom",
                                                                                                        },
                                                                                                        tag: 0,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                            ),
                                                                                                            start: 24,
                                                                                                            end: 33,
                                                                                                            as_str(): "Bar: Zoom",
                                                                                                        },
                                                                                                        attributes: {},
                                                                                                    },
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    31631,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 146,
                                                                                                    end: 168,
                                                                                                    as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                                                },
                                                                                            },
                                                                                            variant: TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 54,
                                                                                                        end: 57,
                                                                                                        as_str(): "Wow",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    33,
                                                                                                ),
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 59,
                                                                                                    end: 62,
                                                                                                    as_str(): "u32",
                                                                                                },
                                                                                                tag: 0,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 54,
                                                                                                    end: 62,
                                                                                                    as_str(): "Wow: u32",
                                                                                                },
                                                                                                attributes: {},
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            33,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 155,
                                                                                            end: 167,
                                                                                            as_str(): "Zoom::Wow(x)",
                                                                                        },
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                    return_type: TypeId(
                                                                                        33,
                                                                                    ),
                                                                                    type_ascription: TypeId(
                                                                                        33,
                                                                                    ),
                                                                                    type_ascription_span: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 165,
                                                                            end: 166,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 165,
                                                                                            end: 166,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 173,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    33,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 173,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 173,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 173,
                                                            as_str(): "x",
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
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 165,
                                                                                                end: 166,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        body: TyExpression {
                                                                                            expression: UnsafeDowncast {
                                                                                                exp: TyExpression {
                                                                                                    expression: UnsafeDowncast {
                                                                                                        exp: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 134,
                                                                                                                        end: 135,
                                                                                                                        as_str(): "x",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 134,
                                                                                                                    end: 135,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                31634,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                ),
                                                                                                                start: 134,
                                                                                                                end: 135,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                        },
                                                                                                        variant: TyEnumVariant {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 24,
                                                                                                                    end: 27,
                                                                                                                    as_str(): "Bar",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_id: TypeId(
                                                                                                                31631,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                31630,
                                                                                                            ),
                                                                                                            type_span: Span {
                                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                ),
                                                                                                                start: 29,
                                                                                                                end: 33,
                                                                                                                as_str(): "Zoom",
                                                                                                            },
                                                                                                            tag: 0,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                                ),
                                                                                                                start: 24,
                                                                                                                end: 33,
                                                                                                                as_str(): "Bar: Zoom",
                                                                                                            },
                                                                                                            attributes: {},
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        31631,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 146,
                                                                                                        end: 168,
                                                                                                        as_str(): "Foo::Bar(Zoom::Wow(x))",
                                                                                                    },
                                                                                                },
                                                                                                variant: TyEnumVariant {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                            ),
                                                                                                            start: 54,
                                                                                                            end: 57,
                                                                                                            as_str(): "Wow",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    type_id: TypeId(
                                                                                                        33,
                                                                                                    ),
                                                                                                    initial_type_id: TypeId(
                                                                                                        33,
                                                                                                    ),
                                                                                                    type_span: Span {
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 59,
                                                                                                        end: 62,
                                                                                                        as_str(): "u32",
                                                                                                    },
                                                                                                    tag: 0,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 54,
                                                                                                        end: 62,
                                                                                                        as_str(): "Wow: u32",
                                                                                                    },
                                                                                                    attributes: {},
                                                                                                },
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                33,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 155,
                                                                                                end: 167,
                                                                                                as_str(): "Zoom::Wow(x)",
                                                                                            },
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                        return_type: TypeId(
                                                                                            33,
                                                                                        ),
                                                                                        type_ascription: TypeId(
                                                                                            33,
                                                                                        ),
                                                                                        type_ascription_span: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 165,
                                                                                end: 166,
                                                                                as_str(): "x",
                                                                            },
                                                                        },
                                                                        TyAstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 165,
                                                                                                end: 166,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 172,
                                                                                            end: 173,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        33,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 173,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 172,
                                                                                end: 173,
                                                                                as_str(): "x",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                33,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 172,
                                                                end: 173,
                                                                as_str(): "x",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 172,
                                                    end: 173,
                                                    as_str(): "x",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 180,
                                            as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            33,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09612d790,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                            ),
                            start: 128,
                            end: 180,
                            as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09612d790,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                    ),
                    start: 128,
                    end: 180,
                    as_str(): "match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09612d790,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
        ),
        start: 67,
        end: 182,
        as_str(): "fn main() -> u32 {\n    let x = Foo::Bar(Zoom::Wow(123));\n    match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        33,
    ),
    initial_return_type: TypeId(
        33,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe09612d790,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
        ),
        start: 80,
        end: 83,
        as_str(): "u32",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

