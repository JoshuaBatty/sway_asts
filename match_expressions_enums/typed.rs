TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0a9b0a660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
            ),
            start: 14,
            end: 15,
            as_str(): "X",
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
                    src (ptr): 0x00007fe0a9b0a660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                    ),
                    start: 22,
                    end: 23,
                    as_str(): "Y",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0a9b0a660,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                ),
                start: 25,
                end: 28,
                as_str(): "u64",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0a9b0a660,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                ),
                start: 22,
                end: 28,
                as_str(): "Y: u64",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0a9b0a660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                    ),
                    start: 34,
                    end: 35,
                    as_str(): "Z",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0a9b0a660,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                ),
                start: 37,
                end: 41,
                as_str(): "bool",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0a9b0a660,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                ),
                start: 34,
                end: 41,
                as_str(): "Z: bool",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0a9b0a660,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
        ),
        start: 9,
        end: 43,
        as_str(): "enum X {\n    Y: u64,\n    Z: bool\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0a9b0a660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
            ),
            start: 48,
            end: 52,
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
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 72,
                                    end: 73,
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
                                                src (ptr): 0x00007fe0a9b0a660,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                ),
                                                start: 14,
                                                end: 15,
                                                as_str(): "X",
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
                                                        src (ptr): 0x00007fe0a9b0a660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                        ),
                                                        start: 22,
                                                        end: 23,
                                                        as_str(): "Y",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    21,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 25,
                                                    end: 28,
                                                    as_str(): "u64",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 22,
                                                    end: 28,
                                                    as_str(): "Y: u64",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a9b0a660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                        ),
                                                        start: 34,
                                                        end: 35,
                                                        as_str(): "Z",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    71,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 37,
                                                    end: 41,
                                                    as_str(): "bool",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 34,
                                                    end: 41,
                                                    as_str(): "Z: bool",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0a9b0a660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 43,
                                            as_str(): "enum X {\n    Y: u64,\n    Z: bool\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a9b0a660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                            ),
                                            start: 22,
                                            end: 23,
                                            as_str(): "Y",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    42,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0a9b0a660,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                ),
                                                start: 81,
                                                end: 83,
                                                as_str(): "42",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0a9b0a660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                        ),
                                        start: 76,
                                        end: 77,
                                        as_str(): "X",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0a9b0a660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                        ),
                                        start: 79,
                                        end: 80,
                                        as_str(): "Y",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0a9b0a660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                            ),
                                            start: 76,
                                            end: 80,
                                            as_str(): "X::Y",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31632,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 79,
                                    end: 80,
                                    as_str(): "Y",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31632,
                            ),
                            type_ascription: TypeId(
                                31631,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0a9b0a660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                    ),
                    start: 68,
                    end: 85,
                    as_str(): "let a = X::Y(42);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 95,
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
                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 105,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 72,
                                                                            end: 73,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                        ),
                                                                        start: 104,
                                                                        end: 105,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31632,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 105,
                                                                    as_str(): "a",
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
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 98,
                                                    end: 191,
                                                    as_str(): "match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
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
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 104,
                                                                                    end: 105,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 104,
                                                                                    end: 105,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 104,
                                                                                end: 105,
                                                                                as_str(): "a",
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
                                                                                    src (ptr): 0x00007fe0b25b74d0,
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
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 104,
                                                                                                    end: 105,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 104,
                                                                                                end: 105,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31632,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 104,
                                                                                            end: 105,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 104,
                                                                                    end: 105,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b25b74d0,
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
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 104,
                                                                                    end: 105,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13318,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0b25b74d0,
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
                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 105,
                                                                    as_str(): "a",
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
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 121,
                                                                                                    end: 123,
                                                                                                    as_str(): "hi",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            body: TyExpression {
                                                                                                expression: UnsafeDowncast {
                                                                                                    exp: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "__match_return_var_name_1",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 104,
                                                                                                                    end: 105,
                                                                                                                    as_str(): "a",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 104,
                                                                                                                end: 105,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            31632,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 104,
                                                                                                            end: 105,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                    },
                                                                                                    variant: TyEnumVariant {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 22,
                                                                                                                end: 23,
                                                                                                                as_str(): "Y",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        type_id: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        initial_type_id: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        type_span: Span {
                                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 25,
                                                                                                            end: 28,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                        tag: 0,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 22,
                                                                                                            end: 28,
                                                                                                            as_str(): "Y: u64",
                                                                                                        },
                                                                                                        attributes: {},
                                                                                                    },
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 116,
                                                                                                    end: 124,
                                                                                                    as_str(): "X::Y(hi)",
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
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 121,
                                                                                    end: 123,
                                                                                    as_str(): "hi",
                                                                                },
                                                                            },
                                                                            TyAstNode {
                                                                                content: ImplicitReturnExpression(
                                                                                    TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 121,
                                                                                                    end: 123,
                                                                                                    as_str(): "hi",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 130,
                                                                                                end: 132,
                                                                                                as_str(): "hi",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 130,
                                                                                            end: 132,
                                                                                            as_str(): "hi",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 130,
                                                                                    end: 132,
                                                                                    as_str(): "hi",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                    ),
                                                                    start: 128,
                                                                    end: 134,
                                                                    as_str(): "{ hi }",
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
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 104,
                                                                                                        end: 105,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 104,
                                                                                                        end: 105,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "eq",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 104,
                                                                                                    end: 105,
                                                                                                    as_str(): "a",
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
                                                                                                        src (ptr): 0x00007fe0b25b74d0,
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
                                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 104,
                                                                                                                        end: 105,
                                                                                                                        as_str(): "a",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 104,
                                                                                                                    end: 105,
                                                                                                                    as_str(): "a",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                31632,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 104,
                                                                                                                end: 105,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 104,
                                                                                                        end: 105,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0b25b74d0,
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
                                                                                                            1,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 104,
                                                                                                        end: 105,
                                                                                                        as_str(): "a",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13317,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe0b25b74d0,
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
                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 104,
                                                                                        end: 105,
                                                                                        as_str(): "a",
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
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 144,
                                                                                                        end: 155,
                                                                                                        as_str(): "X::Z(false)",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 144,
                                                                                                        end: 155,
                                                                                                        as_str(): "X::Z(false)",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "eq",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 144,
                                                                                                    end: 155,
                                                                                                    as_str(): "X::Z(false)",
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
                                                                                                        src (ptr): 0x00007fe0b25b74d0,
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
                                                                                                    expression: UnsafeDowncast {
                                                                                                        exp: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_1",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 104,
                                                                                                                        end: 105,
                                                                                                                        as_str(): "a",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 104,
                                                                                                                    end: 105,
                                                                                                                    as_str(): "a",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                31632,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 104,
                                                                                                                end: 105,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                        },
                                                                                                        variant: TyEnumVariant {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 34,
                                                                                                                    end: 35,
                                                                                                                    as_str(): "Z",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_id: TypeId(
                                                                                                                71,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                71,
                                                                                                            ),
                                                                                                            type_span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 37,
                                                                                                                end: 41,
                                                                                                                as_str(): "bool",
                                                                                                            },
                                                                                                            tag: 1,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 34,
                                                                                                                end: 41,
                                                                                                                as_str(): "Z: bool",
                                                                                                            },
                                                                                                            attributes: {},
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        71,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 144,
                                                                                                        end: 155,
                                                                                                        as_str(): "X::Z(false)",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0b25b74d0,
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
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 149,
                                                                                                        end: 154,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        function_decl_id: DeclId(
                                                                                            13316,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe0b25b74d0,
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
                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 144,
                                                                                        end: 155,
                                                                                        as_str(): "X::Z(false)",
                                                                                    },
                                                                                },
                                                                            },
                                                                            return_type: TypeId(
                                                                                71,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 104,
                                                                                end: 155,
                                                                                as_str(): "a {\n        X::Y(hi) => { hi },\n        X::Z(false)",
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
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 161,
                                                                                                        end: 162,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 161,
                                                                                                end: 162,
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
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 159,
                                                                                end: 164,
                                                                                as_str(): "{ 0 }",
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
                                                                                                                0,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 181,
                                                                                                            end: 182,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 181,
                                                                                                    end: 182,
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
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 179,
                                                                                    end: 184,
                                                                                    as_str(): "{ 0 }",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                        ),
                                                                        start: 159,
                                                                        end: 164,
                                                                        as_str(): "{ 0 }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 134,
                                                            as_str(): "{ hi }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 98,
                                                    end: 191,
                                                    as_str(): "match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 98,
                                    end: 191,
                                    as_str(): "match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31635,
                            ),
                            type_ascription: TypeId(
                                31635,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0a9b0a660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                    ),
                    start: 90,
                    end: 192,
                    as_str(): "let b = match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    };",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a9b0a660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 95,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe0a9b0a660,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                ),
                                start: 202,
                                end: 203,
                                as_str(): "b",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            31635,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0a9b0a660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                            ),
                            start: 202,
                            end: 203,
                            as_str(): "b",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0a9b0a660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                    ),
                    start: 202,
                    end: 203,
                    as_str(): "b",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0a9b0a660,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
        ),
        start: 45,
        end: 205,
        as_str(): "fn main() -> u64 {\n    let a = X::Y(42);\n    let b = match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    };\n    \n    b\n}",
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
        src (ptr): 0x00007fe0a9b0a660,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
        ),
        start: 58,
        end: 61,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

