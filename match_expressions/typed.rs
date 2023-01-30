TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c8b9aa10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
            ),
            start: 12,
            end: 16,
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
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 37,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        5,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 40,
                                    end: 41,
                                    as_str(): "5",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7252,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0c8b9aa10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                    ),
                    start: 32,
                    end: 42,
                    as_str(): "let x = 5;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 51,
                                    end: 52,
                                    as_str(): "a",
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
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 61,
                                                                    end: 62,
                                                                    as_str(): "8",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        8,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 61,
                                                                    end: 62,
                                                                    as_str(): "8",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7255,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 55,
                                                    end: 152,
                                                    as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
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
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 74,
                                                                                    as_str(): "8 {\n        7",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 74,
                                                                                    as_str(): "8 {\n        7",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 61,
                                                                                end: 74,
                                                                                as_str(): "8 {\n        7",
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
                                                                                    src (ptr): 0x00007fe0b54b11d0,
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
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "__match_return_var_name_1",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 61,
                                                                                            end: 62,
                                                                                            as_str(): "8",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 61,
                                                                                        end: 62,
                                                                                        as_str(): "8",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 61,
                                                                                    end: 62,
                                                                                    as_str(): "8",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b54b11d0,
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
                                                                                    Numeric(
                                                                                        7,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 73,
                                                                                    end: 74,
                                                                                    as_str(): "7",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        546,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0b54b11d0,
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
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 61,
                                                                    end: 74,
                                                                    as_str(): "8 {\n        7",
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
                                                                                                4,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 80,
                                                                                            end: 81,
                                                                                            as_str(): "4",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 80,
                                                                                    end: 81,
                                                                                    as_str(): "4",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 78,
                                                                    end: 83,
                                                                    as_str(): "{ 4 }",
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
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 61,
                                                                                                end: 94,
                                                                                                as_str(): "8 {\n        7 => { 4 },\n        9",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 61,
                                                                                                end: 94,
                                                                                                as_str(): "8 {\n        7 => { 4 },\n        9",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "eq",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 61,
                                                                                            end: 94,
                                                                                            as_str(): "8 {\n        7 => { 4 },\n        9",
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
                                                                                                src (ptr): 0x00007fe0b54b11d0,
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
                                                                                            expression: VariableExpression {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "__match_return_var_name_1",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 61,
                                                                                                        end: 62,
                                                                                                        as_str(): "8",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 61,
                                                                                                    end: 62,
                                                                                                    as_str(): "8",
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 61,
                                                                                                end: 62,
                                                                                                as_str(): "8",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    (
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0b54b11d0,
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
                                                                                                Numeric(
                                                                                                    9,
                                                                                                ),
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 93,
                                                                                                end: 94,
                                                                                                as_str(): "9",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                function_decl_id: DeclId(
                                                                                    545,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe0b54b11d0,
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
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 61,
                                                                                end: 94,
                                                                                as_str(): "8 {\n        7 => { 4 },\n        9",
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
                                                                                                            5,
                                                                                                        ),
                                                                                                    ),
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 100,
                                                                                                        end: 101,
                                                                                                        as_str(): "5",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 100,
                                                                                                end: 101,
                                                                                                as_str(): "5",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 98,
                                                                                end: 103,
                                                                                as_str(): "{ 5 }",
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
                                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 61,
                                                                                                            end: 114,
                                                                                                            as_str(): "8 {\n        7 => { 4 },\n        9 => { 5 },\n        8",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ops",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 61,
                                                                                                            end: 114,
                                                                                                            as_str(): "8 {\n        7 => { 4 },\n        9 => { 5 },\n        8",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "eq",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 61,
                                                                                                        end: 114,
                                                                                                        as_str(): "8 {\n        7 => { 4 },\n        9 => { 5 },\n        8",
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
                                                                                                            src (ptr): 0x00007fe0b54b11d0,
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
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "__match_return_var_name_1",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 61,
                                                                                                                    end: 62,
                                                                                                                    as_str(): "8",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 61,
                                                                                                                end: 62,
                                                                                                                as_str(): "8",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 61,
                                                                                                            end: 62,
                                                                                                            as_str(): "8",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0b54b11d0,
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
                                                                                                            Numeric(
                                                                                                                8,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 113,
                                                                                                            end: 114,
                                                                                                            as_str(): "8",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            function_decl_id: DeclId(
                                                                                                544,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe0b54b11d0,
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
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 61,
                                                                                            end: 114,
                                                                                            as_str(): "8 {\n        7 => { 4 },\n        9 => { 5 },\n        8",
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
                                                                                                                        6,
                                                                                                                    ),
                                                                                                                ),
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 120,
                                                                                                                    end: 121,
                                                                                                                    as_str(): "6",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 120,
                                                                                                            end: 121,
                                                                                                            as_str(): "6",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 118,
                                                                                            end: 123,
                                                                                            as_str(): "{ 6 }",
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
                                                                                                                            100,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    return_type: TypeId(
                                                                                                                        21,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 140,
                                                                                                                        end: 143,
                                                                                                                        as_str(): "100",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 140,
                                                                                                                end: 143,
                                                                                                                as_str(): "100",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 138,
                                                                                                end: 145,
                                                                                                as_str(): "{ 100 }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 118,
                                                                                    end: 123,
                                                                                    as_str(): "{ 6 }",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                        ),
                                                                        start: 98,
                                                                        end: 103,
                                                                        as_str(): "{ 5 }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 78,
                                                            end: 83,
                                                            as_str(): "{ 4 }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 55,
                                                    end: 152,
                                                    as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 55,
                                    end: 152,
                                    as_str(): "match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7254,
                            ),
                            type_ascription: TypeId(
                                7254,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0c8b9aa10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                    ),
                    start: 47,
                    end: 153,
                    as_str(): "let a = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 162,
                                    end: 163,
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
                                                                    "__match_return_var_name_2",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 172,
                                                                    end: 173,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 36,
                                                                            end: 37,
                                                                            as_str(): "x",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 173,
                                                                        as_str(): "x",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 172,
                                                                    end: 173,
                                                                    as_str(): "x",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7272,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7272,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 223,
                                                    as_str(): "match x {\n        5 => { 42 },\n        _ => { 24 },\n    }",
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
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 185,
                                                                                    as_str(): "x {\n        5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 185,
                                                                                    as_str(): "x {\n        5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 172,
                                                                                end: 185,
                                                                                as_str(): "x {\n        5",
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
                                                                                    src (ptr): 0x00007fe0b54b11d0,
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
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "__match_return_var_name_2",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 172,
                                                                                            end: 173,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 173,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7272,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 173,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b54b11d0,
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
                                                                                    Numeric(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    7272,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 185,
                                                                                    as_str(): "5",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        547,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0b54b11d0,
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
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 172,
                                                                    end: 185,
                                                                    as_str(): "x {\n        5",
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
                                                                                                42,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 191,
                                                                                            end: 193,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 191,
                                                                                    end: 193,
                                                                                    as_str(): "42",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 189,
                                                                    end: 195,
                                                                    as_str(): "{ 42 }",
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
                                                                                                    24,
                                                                                                ),
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 212,
                                                                                                end: 214,
                                                                                                as_str(): "24",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 212,
                                                                                        end: 214,
                                                                                        as_str(): "24",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                        ),
                                                                        start: 210,
                                                                        end: 216,
                                                                        as_str(): "{ 24 }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 189,
                                                            end: 195,
                                                            as_str(): "{ 42 }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 223,
                                                    as_str(): "match x {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c8b9aa10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                    ),
                                    start: 166,
                                    end: 223,
                                    as_str(): "match x {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7271,
                            ),
                            type_ascription: TypeId(
                                7271,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0c8b9aa10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                    ),
                    start: 158,
                    end: 224,
                    as_str(): "let b = match x {\n        5 => { 42 },\n        _ => { 24 },\n    };",
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
                                                            "__match_return_var_name_3",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 235,
                                                            end: 237,
                                                            as_str(): "42",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    body: TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 235,
                                                            end: 237,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    type_ascription: TypeId(
                                                        7281,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c8b9aa10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                            ),
                                            start: 229,
                                            end: 290,
                                            as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
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
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 235,
                                                                            end: 249,
                                                                            as_str(): "42 {\n        0",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 235,
                                                                            end: 249,
                                                                            as_str(): "42 {\n        0",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "eq",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                        ),
                                                                        start: 235,
                                                                        end: 249,
                                                                        as_str(): "42 {\n        0",
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
                                                                            src (ptr): 0x00007fe0b54b11d0,
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
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_3",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 235,
                                                                                    end: 237,
                                                                                    as_str(): "42",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 235,
                                                                                end: 237,
                                                                                as_str(): "42",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 235,
                                                                            end: 237,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b54b11d0,
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
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 248,
                                                                            end: 249,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                548,
                                                                Span {
                                                                    src (ptr): 0x00007fe0b54b11d0,
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
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 235,
                                                            end: 249,
                                                            as_str(): "42 {\n        0",
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
                                                                                        24,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 255,
                                                                                    end: 257,
                                                                                    as_str(): "24",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 255,
                                                                            end: 257,
                                                                            as_str(): "24",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 259,
                                                            as_str(): "{ 24 }",
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
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 269,
                                                                                                end: 272,
                                                                                                as_str(): "foo",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        body: TyExpression {
                                                                                            expression: VariableExpression {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "__match_return_var_name_3",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 235,
                                                                                                        end: 237,
                                                                                                        as_str(): "42",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 235,
                                                                                                    end: 237,
                                                                                                    as_str(): "42",
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 235,
                                                                                                end: 237,
                                                                                                as_str(): "42",
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
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 269,
                                                                                end: 272,
                                                                                as_str(): "foo",
                                                                            },
                                                                        },
                                                                        TyAstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 269,
                                                                                                end: 272,
                                                                                                as_str(): "foo",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 278,
                                                                                            end: 281,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                        ),
                                                                                        start: 278,
                                                                                        end: 281,
                                                                                        as_str(): "foo",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 278,
                                                                                end: 281,
                                                                                as_str(): "foo",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 276,
                                                                end: 283,
                                                                as_str(): "{ foo }",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                    ),
                                                    start: 253,
                                                    end: 259,
                                                    as_str(): "{ 24 }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c8b9aa10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                            ),
                                            start: 229,
                                            end: 290,
                                            as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c8b9aa10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                            ),
                            start: 229,
                            end: 290,
                            as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c8b9aa10,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                    ),
                    start: 229,
                    end: 290,
                    as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c8b9aa10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
        ),
        start: 9,
        end: 292,
        as_str(): "fn main() -> u64 {\n    let x = 5;\n    let a = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let b = match x {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }\n}",
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
        src (ptr): 0x00007fe0c8b9aa10,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
        ),
        start: 22,
        end: 25,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

