[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 13,
                            end: 17,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 19,
                            end: 22,
                            as_str(): "ops",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 24,
                            end: 26,
                            as_str(): "Eq",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe08b199700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
            ),
            start: 9,
            end: 27,
            as_str(): "use core::ops::Eq;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 32,
                            end: 35,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 37,
                            end: 43,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe08b199700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
            ),
            start: 28,
            end: 47,
            as_str(): "use std::assert::*;",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 54,
                            end: 65,
                            as_str(): "Initialized",
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
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 72,
                                    end: 76,
                                    as_str(): "True",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe08b199700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                ),
                                start: 78,
                                end: 80,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe08b199700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                ),
                                start: 72,
                                end: 80,
                                as_str(): "True: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 86,
                                    end: 91,
                                    as_str(): "False",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe08b199700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                ),
                                start: 93,
                                end: 95,
                                as_str(): "()",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe08b199700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                ),
                                start: 86,
                                end: 95,
                                as_str(): "False: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe08b199700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                        ),
                        start: 49,
                        end: 98,
                        as_str(): "enum Initialized {\n    True: (),\n    False: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08b199700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
            ),
            start: 49,
            end: 98,
            as_str(): "enum Initialized {\n    True: (),\n    False: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe08b199700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                ),
                                start: 105,
                                end: 107,
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
                                src (ptr): 0x00007fe08b199700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                ),
                                start: 112,
                                end: 123,
                                as_str(): "Initialized",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe08b199700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                        ),
                        start: 112,
                        end: 123,
                        as_str(): "Initialized",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 133,
                                    end: 135,
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
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 179,
                                                                                    end: 192,
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
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 180,
                                                                                                        end: 184,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 180,
                                                                                                end: 184,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 186,
                                                                                                        end: 191,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 186,
                                                                                                end: 191,
                                                                                                as_str(): "other",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 179,
                                                                                    end: 192,
                                                                                    as_str(): "(self, other)",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b199700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                    ),
                                                                    start: 173,
                                                                    end: 350,
                                                                    as_str(): "match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }",
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
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 179,
                                                                                                end: 192,
                                                                                                as_str(): "(self, other)",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 179,
                                                                                        end: 192,
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
                                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 208,
                                                                                                                    end: 219,
                                                                                                                    as_str(): "Initialized",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                ),
                                                                                                                start: 221,
                                                                                                                end: 225,
                                                                                                                as_str(): "True",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: CatchAll {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                            ),
                                                                                                            start: 208,
                                                                                                            end: 225,
                                                                                                            as_str(): "Initialized::True",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 208,
                                                                                                        end: 225,
                                                                                                        as_str(): "Initialized::True",
                                                                                                    },
                                                                                                },
                                                                                                EnumScrutinee {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 227,
                                                                                                                    end: 238,
                                                                                                                    as_str(): "Initialized",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                ),
                                                                                                                start: 240,
                                                                                                                end: 244,
                                                                                                                as_str(): "True",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: CatchAll {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                            ),
                                                                                                            start: 227,
                                                                                                            end: 244,
                                                                                                            as_str(): "Initialized::True",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 227,
                                                                                                        end: 244,
                                                                                                        as_str(): "Initialized::True",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 207,
                                                                                                end: 245,
                                                                                                as_str(): "(Initialized::True, Initialized::True)",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: Literal(
                                                                                                Boolean(
                                                                                                    true,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 249,
                                                                                                end: 253,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                            ),
                                                                                            start: 207,
                                                                                            end: 254,
                                                                                            as_str(): "(Initialized::True, Initialized::True) => true,",
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
                                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 268,
                                                                                                                    end: 279,
                                                                                                                    as_str(): "Initialized",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                ),
                                                                                                                start: 281,
                                                                                                                end: 286,
                                                                                                                as_str(): "False",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: CatchAll {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                            ),
                                                                                                            start: 268,
                                                                                                            end: 286,
                                                                                                            as_str(): "Initialized::False",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 268,
                                                                                                        end: 286,
                                                                                                        as_str(): "Initialized::False",
                                                                                                    },
                                                                                                },
                                                                                                EnumScrutinee {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 288,
                                                                                                                    end: 299,
                                                                                                                    as_str(): "Initialized",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ],
                                                                                                        suffix: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                ),
                                                                                                                start: 301,
                                                                                                                end: 306,
                                                                                                                as_str(): "False",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: false,
                                                                                                    },
                                                                                                    value: CatchAll {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                            ),
                                                                                                            start: 288,
                                                                                                            end: 306,
                                                                                                            as_str(): "Initialized::False",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 288,
                                                                                                        end: 306,
                                                                                                        as_str(): "Initialized::False",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 267,
                                                                                                end: 307,
                                                                                                as_str(): "(Initialized::False, Initialized::False)",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: Literal(
                                                                                                Boolean(
                                                                                                    true,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 311,
                                                                                                end: 315,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                            ),
                                                                                            start: 267,
                                                                                            end: 316,
                                                                                            as_str(): "(Initialized::False, Initialized::False) => true,",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: CatchAll {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 329,
                                                                                                end: 330,
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
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 334,
                                                                                                end: 339,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                            ),
                                                                                            start: 329,
                                                                                            end: 340,
                                                                                            as_str(): "_ => false,",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 173,
                                                                            end: 350,
                                                                            as_str(): "match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b199700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                    ),
                                                                    start: 173,
                                                                    end: 350,
                                                                    as_str(): "match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 173,
                                                            end: 350,
                                                            as_str(): "match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 173,
                                                    end: 350,
                                                    as_str(): "match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 173,
                                            end: 350,
                                            as_str(): "match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 163,
                                    end: 356,
                                    as_str(): "{\n        match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 136,
                                            end: 140,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 136,
                                        end: 140,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 142,
                                            end: 147,
                                            as_str(): "other",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 149,
                                        end: 153,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe08b199700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                ),
                                start: 130,
                                end: 356,
                                as_str(): "fn eq(self, other: Self) -> bool {\n        match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe08b199700,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                ),
                                start: 158,
                                end: 162,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe08b199700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                        ),
                        start: 100,
                        end: 358,
                        as_str(): "impl Eq for Initialized {\n    fn eq(self, other: Self) -> bool {\n        match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08b199700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
            ),
            start: 100,
            end: 358,
            as_str(): "impl Eq for Initialized {\n    fn eq(self, other: Self) -> bool {\n        match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }\n    }\n}",
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
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 363,
                            end: 367,
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
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 387,
                                                    end: 388,
                                                    as_str(): "a",
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
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 391,
                                                                            end: 402,
                                                                            as_str(): "Initialized",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b199700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                        ),
                                                                        start: 404,
                                                                        end: 408,
                                                                        as_str(): "True",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 404,
                                                                end: 408,
                                                                as_str(): "True",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 391,
                                                    end: 408,
                                                    as_str(): "Initialized::True",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 383,
                                    end: 409,
                                    as_str(): "let a = Initialized::True;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 418,
                                                    end: 419,
                                                    as_str(): "b",
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
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 422,
                                                                            end: 433,
                                                                            as_str(): "Initialized",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b199700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                        ),
                                                                        start: 435,
                                                                        end: 440,
                                                                        as_str(): "False",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 435,
                                                                end: 440,
                                                                as_str(): "False",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 422,
                                                    end: 440,
                                                    as_str(): "Initialized::False",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 414,
                                    end: 441,
                                    as_str(): "let b = Initialized::False;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 450,
                                                    end: 451,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fe08b199700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                ),
                                                                                start: 456,
                                                                                end: 458,
                                                                                as_str(): "==",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "ops",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b199700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                ),
                                                                                start: 456,
                                                                                end: 458,
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
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 456,
                                                                            end: 458,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: true,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 456,
                                                                end: 458,
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
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 454,
                                                                            end: 455,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b199700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                    ),
                                                                    start: 454,
                                                                    end: 455,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 459,
                                                                            end: 460,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b199700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                    ),
                                                                    start: 459,
                                                                    end: 460,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 454,
                                                    end: 460,
                                                    as_str(): "a == b",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 446,
                                    end: 461,
                                    as_str(): "let c = a == b;",
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
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 466,
                                                                end: 472,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe08b199700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                        ),
                                                        start: 466,
                                                        end: 472,
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
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 475,
                                                                                        end: 477,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 475,
                                                                                        end: 477,
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
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 475,
                                                                                    end: 477,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b199700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                        ),
                                                                        start: 475,
                                                                        end: 477,
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
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 473,
                                                                                    end: 474,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 473,
                                                                            end: 474,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                false,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 478,
                                                                            end: 483,
                                                                            as_str(): "false",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 473,
                                                            end: 483,
                                                            as_str(): "c == false",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 466,
                                            end: 484,
                                            as_str(): "assert(c == false)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 466,
                                    end: 484,
                                    as_str(): "assert(c == false)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 491,
                                            end: 492,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b199700,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                    ),
                                    start: 491,
                                    end: 492,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 377,
                            end: 494,
                            as_str(): "{\n    let a = Initialized::True;\n    let b = Initialized::False;\n    let c = a == b;\n    assert(c == false);\n\n    1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe08b199700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                        ),
                        start: 360,
                        end: 494,
                        as_str(): "fn main() -> u64 {\n    let a = Initialized::True;\n    let b = Initialized::False;\n    let c = a == b;\n    assert(c == false);\n\n    1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe08b199700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                        ),
                        start: 373,
                        end: 376,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08b199700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
            ),
            start: 360,
            end: 494,
            as_str(): "fn main() -> u64 {\n    let a = Initialized::True;\n    let b = Initialized::False;\n    let c = a == b;\n    assert(c == false);\n\n    1\n}",
        },
    },
]
