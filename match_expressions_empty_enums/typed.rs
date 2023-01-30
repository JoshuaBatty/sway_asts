TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0a5ccace0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
            ),
            start: 14,
            end: 21,
            as_str(): "MyNever",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    attributes: {},
    variants: [],
    span: Span {
        src (ptr): 0x00007fe0a5ccace0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
        ),
        start: 9,
        end: 24,
        as_str(): "enum MyNever {}",
    },
    visibility: Private,
}
ImplTrait(
    DeclId(
        13316,
        Span {
            src (ptr): 0x00007fe0a5ccace0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
            ),
            start: 26,
            end: 102,
            as_str(): "impl MyNever {\n    fn into_any<T>(self) -> T {\n        match self {}\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0a5ccace0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
            ),
            start: 107,
            end: 121,
            as_str(): "result_into_ok",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
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
                                                            "__match_return_var_name_2",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 170,
                                                            as_str(): "res",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 125,
                                                                    end: 128,
                                                                    as_str(): "res",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                ),
                                                                start: 167,
                                                                end: 170,
                                                                as_str(): "res",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31668,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 170,
                                                            as_str(): "res",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        31668,
                                                    ),
                                                    type_ascription: TypeId(
                                                        31694,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 161,
                                            end: 418,
                                            as_str(): "match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }",
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
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 167,
                                                                            end: 170,
                                                                            as_str(): "res",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 167,
                                                                            end: 170,
                                                                            as_str(): "res",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "eq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 167,
                                                                        end: 170,
                                                                        as_str(): "res",
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
                                                                            src (ptr): 0x00007fe0c151d270,
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
                                                                                            "__match_return_var_name_2",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 167,
                                                                                            end: 170,
                                                                                            as_str(): "res",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 167,
                                                                                        end: 170,
                                                                                        as_str(): "res",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31668,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 167,
                                                                                    end: 170,
                                                                                    as_str(): "res",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 167,
                                                                            end: 170,
                                                                            as_str(): "res",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c151d270,
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
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 167,
                                                                            end: 170,
                                                                            as_str(): "res",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13334,
                                                                Span {
                                                                    src (ptr): 0x00007fe0c151d270,
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
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 170,
                                                            as_str(): "res",
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
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 192,
                                                                                            end: 193,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    body: TyExpression {
                                                                                        expression: UnsafeDowncast {
                                                                                            exp: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_2",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 167,
                                                                                                            end: 170,
                                                                                                            as_str(): "res",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 167,
                                                                                                        end: 170,
                                                                                                        as_str(): "res",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    31668,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 167,
                                                                                                    end: 170,
                                                                                                    as_str(): "res",
                                                                                                },
                                                                                            },
                                                                                            variant: TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ae5687e0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1862,
                                                                                                        end: 1864,
                                                                                                        as_str(): "Ok",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    31696,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    7487,
                                                                                                ),
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fe0ae5687e0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1866,
                                                                                                    end: 1867,
                                                                                                    as_str(): "T",
                                                                                                },
                                                                                                tag: 0,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ae5687e0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1862,
                                                                                                    end: 1867,
                                                                                                    as_str(): "Ok: T",
                                                                                                },
                                                                                                attributes: {
                                                                                                    DocComment: [
                                                                                                        Attribute {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "doc-comment",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ae5687e0,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                    ),
                                                                                                                    start: 1827,
                                                                                                                    end: 1857,
                                                                                                                    as_str(): "/// Contains the success value",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            args: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ae5687e0,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                        ),
                                                                                                                        start: 1830,
                                                                                                                        end: 1857,
                                                                                                                        as_str(): " Contains the success value",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ae5687e0,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                ),
                                                                                                                start: 1827,
                                                                                                                end: 1857,
                                                                                                                as_str(): "/// Contains the success value",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31696,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 181,
                                                                                            end: 194,
                                                                                            as_str(): "Result::Ok(t)",
                                                                                        },
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                    return_type: TypeId(
                                                                                        31696,
                                                                                    ),
                                                                                    type_ascription: TypeId(
                                                                                        31696,
                                                                                    ),
                                                                                    type_ascription_span: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 192,
                                                                            end: 193,
                                                                            as_str(): "t",
                                                                        },
                                                                    },
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 192,
                                                                                            end: 193,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 198,
                                                                                        end: 199,
                                                                                        as_str(): "t",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31696,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 198,
                                                                                    end: 199,
                                                                                    as_str(): "t",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 198,
                                                                            end: 199,
                                                                            as_str(): "t",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31696,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 198,
                                                            end: 199,
                                                            as_str(): "t",
                                                        },
                                                    },
                                                    else: Some(
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
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 167,
                                                                                        end: 170,
                                                                                        as_str(): "res",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 167,
                                                                                        end: 170,
                                                                                        as_str(): "res",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 167,
                                                                                    end: 170,
                                                                                    as_str(): "res",
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
                                                                                        src (ptr): 0x00007fe0c151d270,
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
                                                                                                        "__match_return_var_name_2",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 167,
                                                                                                        end: 170,
                                                                                                        as_str(): "res",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 167,
                                                                                                    end: 170,
                                                                                                    as_str(): "res",
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                31668,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 167,
                                                                                                end: 170,
                                                                                                as_str(): "res",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 167,
                                                                                        end: 170,
                                                                                        as_str(): "res",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c151d270,
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
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 167,
                                                                                        end: 170,
                                                                                        as_str(): "res",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            13333,
                                                                            Span {
                                                                                src (ptr): 0x00007fe0c151d270,
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
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 167,
                                                                        end: 170,
                                                                        as_str(): "res",
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
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 387,
                                                                                                        end: 392,
                                                                                                        as_str(): "never",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                body: TyExpression {
                                                                                                    expression: UnsafeDowncast {
                                                                                                        exp: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_2",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 167,
                                                                                                                        end: 170,
                                                                                                                        as_str(): "res",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 167,
                                                                                                                    end: 170,
                                                                                                                    as_str(): "res",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                31668,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 167,
                                                                                                                end: 170,
                                                                                                                as_str(): "res",
                                                                                                            },
                                                                                                        },
                                                                                                        variant: TyEnumVariant {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ae5687e0,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                    ),
                                                                                                                    start: 1906,
                                                                                                                    end: 1909,
                                                                                                                    as_str(): "Err",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            type_id: TypeId(
                                                                                                                31702,
                                                                                                            ),
                                                                                                            initial_type_id: TypeId(
                                                                                                                7488,
                                                                                                            ),
                                                                                                            type_span: Span {
                                                                                                                src (ptr): 0x00007fe0ae5687e0,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                ),
                                                                                                                start: 1911,
                                                                                                                end: 1912,
                                                                                                                as_str(): "E",
                                                                                                            },
                                                                                                            tag: 1,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ae5687e0,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                ),
                                                                                                                start: 1906,
                                                                                                                end: 1912,
                                                                                                                as_str(): "Err: E",
                                                                                                            },
                                                                                                            attributes: {
                                                                                                                DocComment: [
                                                                                                                    Attribute {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "doc-comment",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ae5687e0,
                                                                                                                                path: Some(
                                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                                ),
                                                                                                                                start: 1873,
                                                                                                                                end: 1901,
                                                                                                                                as_str(): "/// Contains the error value",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        args: [
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ae5687e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1876,
                                                                                                                                    end: 1901,
                                                                                                                                    as_str(): " Contains the error value",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        ],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ae5687e0,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                            ),
                                                                                                                            start: 1873,
                                                                                                                            end: 1901,
                                                                                                                            as_str(): "/// Contains the error value",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        31702,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 375,
                                                                                                        end: 393,
                                                                                                        as_str(): "Result::Err(never)",
                                                                                                    },
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                                return_type: TypeId(
                                                                                                    31702,
                                                                                                ),
                                                                                                type_ascription: TypeId(
                                                                                                    31702,
                                                                                                ),
                                                                                                type_ascription_span: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 387,
                                                                                        end: 392,
                                                                                        as_str(): "never",
                                                                                    },
                                                                                },
                                                                                TyAstNode {
                                                                                    content: Declaration(
                                                                                        VariableDeclaration(
                                                                                            TyVariableDeclaration {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "__match_return_var_name_3",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 403,
                                                                                                        end: 408,
                                                                                                        as_str(): "never",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                body: TyExpression {
                                                                                                    expression: VariableExpression {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 387,
                                                                                                                end: 392,
                                                                                                                as_str(): "never",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 403,
                                                                                                            end: 408,
                                                                                                            as_str(): "never",
                                                                                                        },
                                                                                                        mutability: Immutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        31702,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 403,
                                                                                                        end: 408,
                                                                                                        as_str(): "never",
                                                                                                    },
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                                return_type: TypeId(
                                                                                                    31702,
                                                                                                ),
                                                                                                type_ascription: TypeId(
                                                                                                    31706,
                                                                                                ),
                                                                                                type_ascription_span: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 397,
                                                                                        end: 411,
                                                                                        as_str(): "match never {}",
                                                                                    },
                                                                                },
                                                                                TyAstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        TyExpression {
                                                                                            expression: IfExp {
                                                                                                condition: TyExpression {
                                                                                                    expression: Literal(
                                                                                                        Boolean(
                                                                                                            true,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        71,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 397,
                                                                                                        end: 411,
                                                                                                        as_str(): "match never {}",
                                                                                                    },
                                                                                                },
                                                                                                then: TyExpression {
                                                                                                    expression: Tuple {
                                                                                                        fields: [],
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        31705,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 397,
                                                                                                        end: 411,
                                                                                                        as_str(): "match never {}",
                                                                                                    },
                                                                                                },
                                                                                                else: Some(
                                                                                                    TyExpression {
                                                                                                        expression: Tuple {
                                                                                                            fields: [],
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            31705,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 397,
                                                                                                            end: 411,
                                                                                                            as_str(): "match never {}",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                31705,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 397,
                                                                                                end: 411,
                                                                                                as_str(): "match never {}",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 397,
                                                                                        end: 411,
                                                                                        as_str(): "match never {}",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        31705,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 397,
                                                                        end: 411,
                                                                        as_str(): "match never {}",
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
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 387,
                                                                                                            end: 392,
                                                                                                            as_str(): "never",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    body: TyExpression {
                                                                                                        expression: UnsafeDowncast {
                                                                                                            exp: TyExpression {
                                                                                                                expression: VariableExpression {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "__match_return_var_name_2",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 167,
                                                                                                                            end: 170,
                                                                                                                            as_str(): "res",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 167,
                                                                                                                        end: 170,
                                                                                                                        as_str(): "res",
                                                                                                                    },
                                                                                                                    mutability: Immutable,
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    31668,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 167,
                                                                                                                    end: 170,
                                                                                                                    as_str(): "res",
                                                                                                                },
                                                                                                            },
                                                                                                            variant: TyEnumVariant {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ae5687e0,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                        ),
                                                                                                                        start: 1906,
                                                                                                                        end: 1909,
                                                                                                                        as_str(): "Err",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                type_id: TypeId(
                                                                                                                    31702,
                                                                                                                ),
                                                                                                                initial_type_id: TypeId(
                                                                                                                    7488,
                                                                                                                ),
                                                                                                                type_span: Span {
                                                                                                                    src (ptr): 0x00007fe0ae5687e0,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                    ),
                                                                                                                    start: 1911,
                                                                                                                    end: 1912,
                                                                                                                    as_str(): "E",
                                                                                                                },
                                                                                                                tag: 1,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ae5687e0,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                    ),
                                                                                                                    start: 1906,
                                                                                                                    end: 1912,
                                                                                                                    as_str(): "Err: E",
                                                                                                                },
                                                                                                                attributes: {
                                                                                                                    DocComment: [
                                                                                                                        Attribute {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "doc-comment",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ae5687e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1873,
                                                                                                                                    end: 1901,
                                                                                                                                    as_str(): "/// Contains the error value",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            args: [
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0ae5687e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1876,
                                                                                                                                        end: 1901,
                                                                                                                                        as_str(): " Contains the error value",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ],
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ae5687e0,
                                                                                                                                path: Some(
                                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                                ),
                                                                                                                                start: 1873,
                                                                                                                                end: 1901,
                                                                                                                                as_str(): "/// Contains the error value",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            31702,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 375,
                                                                                                            end: 393,
                                                                                                            as_str(): "Result::Err(never)",
                                                                                                        },
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                    return_type: TypeId(
                                                                                                        31702,
                                                                                                    ),
                                                                                                    type_ascription: TypeId(
                                                                                                        31702,
                                                                                                    ),
                                                                                                    type_ascription_span: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 387,
                                                                                            end: 392,
                                                                                            as_str(): "never",
                                                                                        },
                                                                                    },
                                                                                    TyAstNode {
                                                                                        content: Declaration(
                                                                                            VariableDeclaration(
                                                                                                TyVariableDeclaration {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_3",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 403,
                                                                                                            end: 408,
                                                                                                            as_str(): "never",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    body: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 387,
                                                                                                                    end: 392,
                                                                                                                    as_str(): "never",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 403,
                                                                                                                end: 408,
                                                                                                                as_str(): "never",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            31702,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 403,
                                                                                                            end: 408,
                                                                                                            as_str(): "never",
                                                                                                        },
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                    return_type: TypeId(
                                                                                                        31702,
                                                                                                    ),
                                                                                                    type_ascription: TypeId(
                                                                                                        31706,
                                                                                                    ),
                                                                                                    type_ascription_span: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 397,
                                                                                            end: 411,
                                                                                            as_str(): "match never {}",
                                                                                        },
                                                                                    },
                                                                                    TyAstNode {
                                                                                        content: ImplicitReturnExpression(
                                                                                            TyExpression {
                                                                                                expression: IfExp {
                                                                                                    condition: TyExpression {
                                                                                                        expression: Literal(
                                                                                                            Boolean(
                                                                                                                true,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            71,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 397,
                                                                                                            end: 411,
                                                                                                            as_str(): "match never {}",
                                                                                                        },
                                                                                                    },
                                                                                                    then: TyExpression {
                                                                                                        expression: Tuple {
                                                                                                            fields: [],
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            31705,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                            ),
                                                                                                            start: 397,
                                                                                                            end: 411,
                                                                                                            as_str(): "match never {}",
                                                                                                        },
                                                                                                    },
                                                                                                    else: Some(
                                                                                                        TyExpression {
                                                                                                            expression: Tuple {
                                                                                                                fields: [],
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                31705,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 397,
                                                                                                                end: 411,
                                                                                                                as_str(): "match never {}",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    31705,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 397,
                                                                                                    end: 411,
                                                                                                    as_str(): "match never {}",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 397,
                                                                                            end: 411,
                                                                                            as_str(): "match never {}",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            31705,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 397,
                                                                            end: 411,
                                                                            as_str(): "match never {}",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                31705,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                ),
                                                                start: 397,
                                                                end: 411,
                                                                as_str(): "match never {}",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    31696,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a5ccace0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                    ),
                                                    start: 198,
                                                    end: 199,
                                                    as_str(): "t",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 161,
                                            end: 418,
                                            as_str(): "match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            31696,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0a5ccace0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                            ),
                            start: 161,
                            end: 418,
                            as_str(): "match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0a5ccace0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                    ),
                    start: 161,
                    end: 418,
                    as_str(): "match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0a5ccace0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                    ),
                    start: 125,
                    end: 128,
                    as_str(): "res",
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
            type_id: TypeId(
                31668,
            ),
            initial_type_id: TypeId(
                31667,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0a5ccace0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                ),
                start: 130,
                end: 148,
                as_str(): "Result<T, MyNever>",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0a5ccace0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
        ),
        start: 104,
        end: 420,
        as_str(): "fn result_into_ok<T>(res: Result<T, MyNever>) -> T {\n    match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        31666,
    ),
    initial_return_type: TypeId(
        31693,
    ),
    type_parameters: [
        T: TypeId(31666),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0a5ccace0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
        ),
        start: 153,
        end: 154,
        as_str(): "T",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0a5ccace0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
            ),
            start: 425,
            end: 429,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                42,
                            ),
                        ),
                        return_type: TypeId(
                            31709,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0a5ccace0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                            ),
                            start: 445,
                            end: 447,
                            as_str(): "42",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0a5ccace0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                    ),
                    start: 445,
                    end: 447,
                    as_str(): "42",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0a5ccace0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
        ),
        start: 422,
        end: 449,
        as_str(): "fn main() -> u64 {\n    42\n}",
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
        src (ptr): 0x00007fe0a5ccace0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
        ),
        start: 435,
        end: 438,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

