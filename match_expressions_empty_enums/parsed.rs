[
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
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
                    attributes: {},
                    type_parameters: [],
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a5ccace0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
            ),
            start: 9,
            end: 24,
            as_str(): "enum MyNever {}",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0a5ccace0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                ),
                                start: 31,
                                end: 38,
                                as_str(): "MyNever",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0a5ccace0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                        ),
                        start: 31,
                        end: 38,
                        as_str(): "MyNever",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0a5ccace0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                    ),
                                    start: 48,
                                    end: 56,
                                    as_str(): "into_any",
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
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 87,
                                                                                    end: 91,
                                                                                    as_str(): "self",
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
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 87,
                                                                                            end: 91,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 87,
                                                                                    end: 91,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 94,
                                                                    as_str(): "match self {}",
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
                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 87,
                                                                                                end: 91,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 87,
                                                                                        end: 91,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                },
                                                                                branches: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 81,
                                                                            end: 94,
                                                                            as_str(): "match self {}",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 94,
                                                                    as_str(): "match self {}",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 81,
                                                            end: 94,
                                                            as_str(): "match self {}",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0a5ccace0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                    ),
                                                    start: 81,
                                                    end: 94,
                                                    as_str(): "match self {}",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 94,
                                            as_str(): "match self {}",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0a5ccace0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                    ),
                                    start: 71,
                                    end: 100,
                                    as_str(): "{\n        match self {}\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 60,
                                            end: 64,
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
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 60,
                                        end: 64,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0a5ccace0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                ),
                                start: 45,
                                end: 100,
                                as_str(): "fn into_any<T>(self) -> T {\n        match self {}\n    }",
                            },
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 70,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_parameters: [
                                T: TypeId(31628),
                            ],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0a5ccace0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                ),
                                start: 69,
                                end: 70,
                                as_str(): "T",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0a5ccace0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                        ),
                        start: 26,
                        end: 102,
                        as_str(): "impl MyNever {\n    fn into_any<T>(self) -> T {\n        match self {}\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a5ccace0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
            ),
            start: 26,
            end: 102,
            as_str(): "impl MyNever {\n    fn into_any<T>(self) -> T {\n        match self {}\n    }\n}",
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
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
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
                                                                    is_mutable: false,
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
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
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
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 181,
                                                                                                    end: 187,
                                                                                                    as_str(): "Result",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 189,
                                                                                                end: 191,
                                                                                                as_str(): "Ok",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Variable {
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
                                                                                            start: 192,
                                                                                            end: 193,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                    },
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
                                                                                result: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 198,
                                                                                                end: 199,
                                                                                                as_str(): "t",
                                                                                            },
                                                                                            is_raw_ident: false,
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
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 181,
                                                                                    end: 200,
                                                                                    as_str(): "Result::Ok(t) => t,",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 375,
                                                                                                    end: 381,
                                                                                                    as_str(): "Result",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 383,
                                                                                                end: 386,
                                                                                                as_str(): "Err",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Variable {
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
                                                                                            start: 387,
                                                                                            end: 392,
                                                                                            as_str(): "never",
                                                                                        },
                                                                                    },
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
                                                                                result: Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [
                                                                                                AstNode {
                                                                                                    content: Declaration(
                                                                                                        VariableDeclaration(
                                                                                                            VariableDeclaration {
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
                                                                                                                type_ascription: Unknown,
                                                                                                                type_ascription_span: None,
                                                                                                                body: Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
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
                                                                                                                is_mutable: false,
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
                                                                                                AstNode {
                                                                                                    content: ImplicitReturnExpression(
                                                                                                        Expression {
                                                                                                            kind: Match(
                                                                                                                MatchExpression {
                                                                                                                    value: Expression {
                                                                                                                        kind: Variable(
                                                                                                                            BaseIdent {
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
                                                                                                                    branches: [],
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
                                                                                            whole_block_span: Span {
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
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 375,
                                                                                    end: 412,
                                                                                    as_str(): "Result::Err(never) => match never {},",
                                                                                },
                                                                            },
                                                                        ],
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
                                                whole_block_span: Span {
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0a5ccace0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                            ),
                            start: 155,
                            end: 420,
                            as_str(): "{\n    match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
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
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 130,
                                        end: 136,
                                        as_str(): "Result",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                31629,
                                            ),
                                            initial_type_id: TypeId(
                                                31629,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0a5ccace0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                ),
                                                start: 137,
                                                end: 138,
                                                as_str(): "T",
                                            },
                                        },
                                        TypeArgument {
                                            type_id: TypeId(
                                                31630,
                                            ),
                                            initial_type_id: TypeId(
                                                31630,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0a5ccace0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                ),
                                                start: 140,
                                                end: 147,
                                                as_str(): "MyNever",
                                            },
                                        },
                                    ],
                                ),
                            },
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
                    span: Span {
                        src (ptr): 0x00007fe0a5ccace0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                        ),
                        start: 104,
                        end: 420,
                        as_str(): "fn result_into_ok<T>(res: Result<T, MyNever>) -> T {\n    match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0a5ccace0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                ),
                                start: 153,
                                end: 154,
                                as_str(): "T",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [
                        T: TypeId(31631),
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a5ccace0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
            ),
            start: 104,
            end: 420,
            as_str(): "fn result_into_ok<T>(res: Result<T, MyNever>) -> T {\n    match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }\n}",
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                42,
                                            ),
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0a5ccace0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                            ),
                            start: 439,
                            end: 449,
                            as_str(): "{\n    42\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0a5ccace0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                        ),
                        start: 422,
                        end: 449,
                        as_str(): "fn main() -> u64 {\n    42\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0a5ccace0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
            ),
            start: 422,
            end: 449,
            as_str(): "fn main() -> u64 {\n    42\n}",
        },
    },
]
