TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0bd033020,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
            ),
            start: 120,
            end: 124,
            as_str(): "main",
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
                                                            "__match_return_var_name_1",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd033020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                            ),
                                                            start: 147,
                                                            end: 151,
                                                            as_str(): "true",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd033020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                            ),
                                                            start: 147,
                                                            end: 151,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        71,
                                                    ),
                                                    type_ascription: TypeId(
                                                        7252,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bd033020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                            ),
                                            start: 141,
                                            end: 268,
                                            as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
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
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 147,
                                                                            end: 166,
                                                                            as_str(): "true {\n        true",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 147,
                                                                            end: 166,
                                                                            as_str(): "true {\n        true",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "eq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd033020,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                        ),
                                                                        start: 147,
                                                                        end: 166,
                                                                        as_str(): "true {\n        true",
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
                                                                            src (ptr): 0x00007fe0ae4073c0,
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
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_1",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd033020,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                    ),
                                                                                    start: 147,
                                                                                    end: 151,
                                                                                    as_str(): "true",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd033020,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                ),
                                                                                start: 147,
                                                                                end: 151,
                                                                                as_str(): "true",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 147,
                                                                            end: 151,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ae4073c0,
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
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 162,
                                                                            end: 166,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                545,
                                                                Span {
                                                                    src (ptr): 0x00007fe0ae4073c0,
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
                                                            src (ptr): 0x00007fe0bd033020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                            ),
                                                            start: 147,
                                                            end: 166,
                                                            as_str(): "true {\n        true",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Expression(
                                                                            TyExpression {
                                                                                expression: Return(
                                                                                    TyExpression {
                                                                                        expression: Literal(
                                                                                            Boolean(
                                                                                                true,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            71,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd033020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                            ),
                                                                                            start: 191,
                                                                                            end: 195,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    7257,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd033020,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 195,
                                                                                    as_str(): "return true",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 184,
                                                                            end: 195,
                                                                            as_str(): "return true",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            7215,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd033020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 206,
                                                            as_str(): "{\n            return true;\n        }",
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
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 147,
                                                                                        end: 221,
                                                                                        as_str(): "true {\n        true => {\n            return true;\n        },\n        false",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 147,
                                                                                        end: 221,
                                                                                        as_str(): "true {\n        true => {\n            return true;\n        },\n        false",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd033020,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                    ),
                                                                                    start: 147,
                                                                                    end: 221,
                                                                                    as_str(): "true {\n        true => {\n            return true;\n        },\n        false",
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
                                                                                        src (ptr): 0x00007fe0ae4073c0,
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
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_1",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd033020,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                ),
                                                                                                start: 147,
                                                                                                end: 151,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd033020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                            ),
                                                                                            start: 147,
                                                                                            end: 151,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 147,
                                                                                        end: 151,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ae4073c0,
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
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 216,
                                                                                        end: 221,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            544,
                                                                            Span {
                                                                                src (ptr): 0x00007fe0ae4073c0,
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
                                                                        src (ptr): 0x00007fe0bd033020,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                        ),
                                                                        start: 147,
                                                                        end: 221,
                                                                        as_str(): "true {\n        true => {\n            return true;\n        },\n        false",
                                                                    },
                                                                },
                                                                then: TyExpression {
                                                                    expression: CodeBlock(
                                                                        TyCodeBlock {
                                                                            contents: [
                                                                                TyAstNode {
                                                                                    content: Expression(
                                                                                        TyExpression {
                                                                                            expression: Return(
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
                                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                        ),
                                                                                                        start: 246,
                                                                                                        end: 251,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                7261,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0bd033020,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                ),
                                                                                                start: 239,
                                                                                                end: 251,
                                                                                                as_str(): "return false",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 239,
                                                                                        end: 251,
                                                                                        as_str(): "return false",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        7215,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bd033020,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                        ),
                                                                        start: 225,
                                                                        end: 262,
                                                                        as_str(): "{\n            return false;\n        }",
                                                                    },
                                                                },
                                                                else: Some(
                                                                    TyExpression {
                                                                        expression: CodeBlock(
                                                                            TyCodeBlock {
                                                                                contents: [
                                                                                    TyAstNode {
                                                                                        content: Expression(
                                                                                            TyExpression {
                                                                                                expression: Return(
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
                                                                                                            src (ptr): 0x00007fe0bd033020,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                            ),
                                                                                                            start: 246,
                                                                                                            end: 251,
                                                                                                            as_str(): "false",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    7261,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0bd033020,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                    ),
                                                                                                    start: 239,
                                                                                                    end: 251,
                                                                                                    as_str(): "return false",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd033020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                            ),
                                                                                            start: 239,
                                                                                            end: 251,
                                                                                            as_str(): "return false",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            7215,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 225,
                                                                            end: 262,
                                                                            as_str(): "{\n            return false;\n        }",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                7215,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd033020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                ),
                                                                start: 225,
                                                                end: 262,
                                                                as_str(): "{\n            return false;\n        }",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    7215,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd033020,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                    ),
                                                    start: 170,
                                                    end: 206,
                                                    as_str(): "{\n            return true;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bd033020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                            ),
                                            start: 141,
                                            end: 268,
                                            as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            7215,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0bd033020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                            ),
                            start: 141,
                            end: 268,
                            as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0bd033020,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                    ),
                    start: 141,
                    end: 268,
                    as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0bd033020,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
        ),
        start: 117,
        end: 270,
        as_str(): "fn main() -> bool {\n    match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }\n}",
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
        src (ptr): 0x00007fe0bd033020,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
        ),
        start: 130,
        end: 134,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

