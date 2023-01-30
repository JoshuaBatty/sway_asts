

TyEnumDeclaration {
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
    type_parameters: [],
    attributes: {},
    variants: [
        TyEnumVariant {
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
            type_id: TypeId(
                31631,
            ),
            initial_type_id: TypeId(
                31630,
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
            attributes: {},
        },
        TyEnumVariant {
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
            type_id: TypeId(
                31633,
            ),
            initial_type_id: TypeId(
                31632,
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
            attributes: {},
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
}
ImplTrait(
    DeclId(
        13333,
        Span {
            src (ptr): 0x00007fe08b199700,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
            ),
            start: 100,
            end: 358,
            as_str(): "impl Eq for Initialized {\n    fn eq(self, other: Self) -> bool {\n        match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
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
                                        type_parameters: [],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
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
                                                type_id: TypeId(
                                                    31631,
                                                ),
                                                initial_type_id: TypeId(
                                                    31630,
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
                                                attributes: {},
                                            },
                                            TyEnumVariant {
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
                                                type_id: TypeId(
                                                    31633,
                                                ),
                                                initial_type_id: TypeId(
                                                    31632,
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
                                                attributes: {},
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
                                    variant_name: BaseIdent {
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
                                    tag: 0,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 391,
                                        end: 402,
                                        as_str(): "Initialized",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 404,
                                        end: 408,
                                        as_str(): "True",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
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
                                },
                                return_type: TypeId(
                                    31635,
                                ),
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
                            mutability: Immutable,
                            return_type: TypeId(
                                31635,
                            ),
                            type_ascription: TypeId(
                                31709,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
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
                                        type_parameters: [],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
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
                                                type_id: TypeId(
                                                    31631,
                                                ),
                                                initial_type_id: TypeId(
                                                    31630,
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
                                                attributes: {},
                                            },
                                            TyEnumVariant {
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
                                                type_id: TypeId(
                                                    31633,
                                                ),
                                                initial_type_id: TypeId(
                                                    31632,
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
                                                attributes: {},
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
                                    variant_name: BaseIdent {
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
                                    tag: 1,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 422,
                                        end: 433,
                                        as_str(): "Initialized",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 435,
                                        end: 440,
                                        as_str(): "False",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
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
                                },
                                return_type: TypeId(
                                    31635,
                                ),
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
                            mutability: Immutable,
                            return_type: TypeId(
                                31635,
                            ),
                            type_ascription: TypeId(
                                31710,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: FunctionApplication {
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
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
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
                                            TyExpression {
                                                expression: VariableExpression {
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
                                                    span: Span {
                                                        src (ptr): 0x00007fe08b199700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                        ),
                                                        start: 454,
                                                        end: 455,
                                                        as_str(): "a",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    31635,
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
                                        ),
                                        (
                                            BaseIdent {
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
                                            TyExpression {
                                                expression: VariableExpression {
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
                                                    span: Span {
                                                        src (ptr): 0x00007fe08b199700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                        ),
                                                        start: 459,
                                                        end: 460,
                                                        as_str(): "b",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    31635,
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
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13336,
                                        Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 130,
                                            end: 356,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
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
                                    ),
                                },
                                return_type: TypeId(
                                    71,
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
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                31711,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
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
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09699e4b0,
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
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a65b87e0,
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
                                                        expression: VariableExpression {
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
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 473,
                                                                end: 474,
                                                                as_str(): "c",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            71,
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
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a65b87e0,
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
                                                            71,
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
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13338,
                                                Span {
                                                    src (ptr): 0x00007fe0a65b87e0,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
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
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
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
                                ),
                            ],
                            function_decl_id: DeclId(
                                13339,
                                Span {
                                    src (ptr): 0x00007fe09699e4b0,
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
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 466,
                                        end: 472,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31718,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                1,
                            ),
                        ),
                        return_type: TypeId(
                            31719,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe08b199700,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
        ),
        start: 360,
        end: 494,
        as_str(): "fn main() -> u64 {\n    let a = Initialized::True;\n    let b = Initialized::False;\n    let c = a == b;\n    assert(c == false);\n\n    1\n}",
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
        src (ptr): 0x00007fe08b199700,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
        ),
        start: 373,
        end: 376,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

