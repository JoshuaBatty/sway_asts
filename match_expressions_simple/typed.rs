TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe09ea4f9c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 37,
                                    as_str(): "a",
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
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                    src (ptr): 0x00007fe09ea4f9c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                    ),
                    start: 32,
                    end: 42,
                    as_str(): "let a = 5;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 51,
                                    end: 52,
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 61,
                                                                                            end: 62,
                                                                                            as_str(): "8",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 80,
                                                                                            end: 81,
                                                                                            as_str(): "4",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe096926ee0,
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
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 61,
                                                                                                        end: 62,
                                                                                                        as_str(): "8",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe096926ee0,
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                        src (ptr): 0x00007fe096926ee0,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 100,
                                                                                                        end: 101,
                                                                                                        as_str(): "5",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 61,
                                                                                                                    end: 62,
                                                                                                                    as_str(): "8",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 120,
                                                                                                                    end: 121,
                                                                                                                    as_str(): "6",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 140,
                                                                                                                        end: 143,
                                                                                                                        as_str(): "100",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 78,
                                                            end: 83,
                                                            as_str(): "{ 4 }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                    src (ptr): 0x00007fe09ea4f9c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                    ),
                    start: 47,
                    end: 153,
                    as_str(): "let b = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 162,
                                    end: 163,
                                    as_str(): "c",
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 172,
                                                                    end: 173,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 36,
                                                                            end: 37,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 173,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 172,
                                                                    end: 173,
                                                                    as_str(): "a",
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
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 223,
                                                    as_str(): "match a {\n        5 => { 42 },\n        _ => { 24 },\n    }",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 185,
                                                                                    as_str(): "a {\n        5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 185,
                                                                                    as_str(): "a {\n        5",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 172,
                                                                                end: 185,
                                                                                as_str(): "a {\n        5",
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
                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 172,
                                                                                            end: 173,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 172,
                                                                                        end: 173,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7272,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 173,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 172,
                                                                    end: 185,
                                                                    as_str(): "a {\n        5",
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 191,
                                                                                            end: 193,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 212,
                                                                                                end: 214,
                                                                                                as_str(): "24",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 189,
                                                            end: 195,
                                                            as_str(): "{ 42 }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 223,
                                                    as_str(): "match a {\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 166,
                                    end: 223,
                                    as_str(): "match a {\n        5 => { 42 },\n        _ => { 24 },\n    }",
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
                    src (ptr): 0x00007fe09ea4f9c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                    ),
                    start: 158,
                    end: 224,
                    as_str(): "let c = match a {\n        5 => { 42 },\n        _ => { 24 },\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 233,
                                    end: 234,
                                    as_str(): "d",
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
                                                                    "__match_return_var_name_3",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 243,
                                                                    end: 245,
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 243,
                                                                    end: 245,
                                                                    as_str(): "42",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7282,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 237,
                                                    end: 298,
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 243,
                                                                                    end: 257,
                                                                                    as_str(): "42 {\n        0",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 243,
                                                                                    end: 257,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 243,
                                                                                end: 257,
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
                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 243,
                                                                                            end: 245,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 243,
                                                                                        end: 245,
                                                                                        as_str(): "42",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 243,
                                                                                    end: 245,
                                                                                    as_str(): "42",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 256,
                                                                                    end: 257,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        548,
                                                                        Span {
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 243,
                                                                    end: 257,
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
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 263,
                                                                                            end: 265,
                                                                                            as_str(): "24",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 263,
                                                                                    end: 265,
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 261,
                                                                    end: 267,
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
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 277,
                                                                                                        end: 280,
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
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 243,
                                                                                                                end: 245,
                                                                                                                as_str(): "42",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 243,
                                                                                                            end: 245,
                                                                                                            as_str(): "42",
                                                                                                        },
                                                                                                        mutability: Immutable,
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        21,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 243,
                                                                                                        end: 245,
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
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 277,
                                                                                        end: 280,
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
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 277,
                                                                                                        end: 280,
                                                                                                        as_str(): "foo",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 286,
                                                                                                    end: 289,
                                                                                                    as_str(): "foo",
                                                                                                },
                                                                                                mutability: Immutable,
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 286,
                                                                                                end: 289,
                                                                                                as_str(): "foo",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 286,
                                                                                        end: 289,
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 284,
                                                                        end: 291,
                                                                        as_str(): "{ foo }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 261,
                                                            end: 267,
                                                            as_str(): "{ 24 }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 237,
                                                    end: 298,
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
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 237,
                                    end: 298,
                                    as_str(): "match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7281,
                            ),
                            type_ascription: TypeId(
                                7281,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09ea4f9c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                    ),
                    start: 229,
                    end: 299,
                    as_str(): "let d = match 42 {\n        0 => { 24 },\n        foo => { foo },\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 308,
                                    end: 309,
                                    as_str(): "e",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [
                                        TyExpression {
                                            expression: Tuple {
                                                fields: [
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 324,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 326,
                                                            end: 327,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ],
                                            },
                                            return_type: TypeId(
                                                7297,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe09ea4f9c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                ),
                                                start: 322,
                                                end: 328,
                                                as_str(): "(1, 2)",
                                            },
                                        },
                                        TyExpression {
                                            expression: Tuple {
                                                fields: [
                                                    TyExpression {
                                                        expression: Tuple {
                                                            fields: [
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 353,
                                                                        end: 354,
                                                                        as_str(): "3",
                                                                    },
                                                                },
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 356,
                                                                        end: 357,
                                                                        as_str(): "4",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        return_type: TypeId(
                                                            7305,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 352,
                                                            end: 358,
                                                            as_str(): "(3, 4)",
                                                        },
                                                    },
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 372,
                                                            end: 373,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ],
                                            },
                                            return_type: TypeId(
                                                7310,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe09ea4f9c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                ),
                                                start: 338,
                                                end: 383,
                                                as_str(): "(\n            (3, 4),\n            5\n        )",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7315,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 312,
                                    end: 389,
                                    as_str(): "(\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    )",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7315,
                            ),
                            type_ascription: TypeId(
                                7290,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09ea4f9c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                    ),
                    start: 304,
                    end: 390,
                    as_str(): "let e = (\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 399,
                                    end: 400,
                                    as_str(): "f",
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
                                                                    "__match_return_var_name_4",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 409,
                                                                    end: 410,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            body: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 308,
                                                                            end: 309,
                                                                            as_str(): "e",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 409,
                                                                        end: 410,
                                                                        as_str(): "e",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7321,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 409,
                                                                    end: 410,
                                                                    as_str(): "e",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                7321,
                                                            ),
                                                            type_ascription: TypeId(
                                                                7317,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 403,
                                                    end: 500,
                                                    as_str(): "match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 422,
                                                                                    end: 428,
                                                                                    as_str(): "(3, _)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 422,
                                                                                    end: 428,
                                                                                    as_str(): "(3, _)",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "eq",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 422,
                                                                                end: 428,
                                                                                as_str(): "(3, _)",
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
                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                expression: TupleElemAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: TupleElemAccess {
                                                                                            prefix: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_4",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 409,
                                                                                                            end: 410,
                                                                                                            as_str(): "e",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 409,
                                                                                                        end: 410,
                                                                                                        as_str(): "e",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7326,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 409,
                                                                                                    end: 410,
                                                                                                    as_str(): "e",
                                                                                                },
                                                                                            },
                                                                                            elem_to_access_num: 0,
                                                                                            resolved_type_of_parent: TypeId(
                                                                                                7326,
                                                                                            ),
                                                                                            elem_to_access_span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 421,
                                                                                                end: 432,
                                                                                                as_str(): "((3, _), _)",
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7323,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 421,
                                                                                            end: 432,
                                                                                            as_str(): "((3, _), _)",
                                                                                        },
                                                                                    },
                                                                                    elem_to_access_num: 0,
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7323,
                                                                                    ),
                                                                                    elem_to_access_span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 422,
                                                                                        end: 428,
                                                                                        as_str(): "(3, _)",
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 422,
                                                                                    end: 428,
                                                                                    as_str(): "(3, _)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 423,
                                                                                    end: 424,
                                                                                    as_str(): "3",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        550,
                                                                        Span {
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 422,
                                                                    end: 428,
                                                                    as_str(): "(3, _)",
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
                                                                                                99,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 438,
                                                                                            end: 440,
                                                                                            as_str(): "99",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 438,
                                                                                    end: 440,
                                                                                    as_str(): "99",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 436,
                                                                    end: 442,
                                                                    as_str(): "{ 99 }",
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 456,
                                                                                                end: 462,
                                                                                                as_str(): "(_, 5)",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "ops",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 456,
                                                                                                end: 462,
                                                                                                as_str(): "(_, 5)",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "eq",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 456,
                                                                                            end: 462,
                                                                                            as_str(): "(_, 5)",
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
                                                                                                src (ptr): 0x00007fe096926ee0,
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
                                                                                            expression: TupleElemAccess {
                                                                                                prefix: TyExpression {
                                                                                                    expression: TupleElemAccess {
                                                                                                        prefix: TyExpression {
                                                                                                            expression: VariableExpression {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "__match_return_var_name_4",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 409,
                                                                                                                        end: 410,
                                                                                                                        as_str(): "e",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 409,
                                                                                                                    end: 410,
                                                                                                                    as_str(): "e",
                                                                                                                },
                                                                                                                mutability: Immutable,
                                                                                                            },
                                                                                                            return_type: TypeId(
                                                                                                                7326,
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 409,
                                                                                                                end: 410,
                                                                                                                as_str(): "e",
                                                                                                            },
                                                                                                        },
                                                                                                        elem_to_access_num: 1,
                                                                                                        resolved_type_of_parent: TypeId(
                                                                                                            7326,
                                                                                                        ),
                                                                                                        elem_to_access_span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 452,
                                                                                                            end: 463,
                                                                                                            as_str(): "(_, (_, 5))",
                                                                                                        },
                                                                                                    },
                                                                                                    return_type: TypeId(
                                                                                                        7325,
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 452,
                                                                                                        end: 463,
                                                                                                        as_str(): "(_, (_, 5))",
                                                                                                    },
                                                                                                },
                                                                                                elem_to_access_num: 1,
                                                                                                resolved_type_of_parent: TypeId(
                                                                                                    7325,
                                                                                                ),
                                                                                                elem_to_access_span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 456,
                                                                                                    end: 462,
                                                                                                    as_str(): "(_, 5)",
                                                                                                },
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 456,
                                                                                                end: 462,
                                                                                                as_str(): "(_, 5)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    (
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe096926ee0,
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
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 460,
                                                                                                end: 461,
                                                                                                as_str(): "5",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                function_decl_id: DeclId(
                                                                                    549,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe096926ee0,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 456,
                                                                                end: 462,
                                                                                as_str(): "(_, 5)",
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
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 469,
                                                                                                        end: 471,
                                                                                                        as_str(): "42",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 469,
                                                                                                end: 471,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 467,
                                                                                end: 473,
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
                                                                                                                0,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 490,
                                                                                                            end: 491,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 490,
                                                                                                    end: 491,
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 488,
                                                                                    end: 493,
                                                                                    as_str(): "{ 0 }",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 467,
                                                                        end: 473,
                                                                        as_str(): "{ 42 }",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 436,
                                                            end: 442,
                                                            as_str(): "{ 99 }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 403,
                                                    end: 500,
                                                    as_str(): "match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 403,
                                    end: 500,
                                    as_str(): "match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7316,
                            ),
                            type_ascription: TypeId(
                                7316,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe09ea4f9c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                    ),
                    start: 395,
                    end: 501,
                    as_str(): "let f = match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    };",
                },
            },
            TyAstNode {
                content: Expression(
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
                                                            "__match_return_var_name_5",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 513,
                                                            end: 517,
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 513,
                                                            end: 517,
                                                            as_str(): "true",
                                                        },
                                                    },
                                                    mutability: Immutable,
                                                    return_type: TypeId(
                                                        71,
                                                    ),
                                                    type_ascription: TypeId(
                                                        7350,
                                                    ),
                                                    type_ascription_span: None,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                            ),
                                            start: 507,
                                            end: 623,
                                            as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 513,
                                                                            end: 532,
                                                                            as_str(): "true {\n        true",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 513,
                                                                            end: 532,
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 513,
                                                                        end: 532,
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
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                                    "__match_return_var_name_5",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 513,
                                                                                    end: 517,
                                                                                    as_str(): "true",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 513,
                                                                                end: 517,
                                                                                as_str(): "true",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 513,
                                                                            end: 517,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 528,
                                                                            end: 532,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                552,
                                                                Span {
                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 513,
                                                            end: 532,
                                                            as_str(): "true {\n        true",
                                                        },
                                                    },
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            TyExpression {
                                                                                expression: Tuple {
                                                                                    fields: [],
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7354,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 536,
                                                                                    end: 538,
                                                                                    as_str(): "()",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 536,
                                                                            end: 538,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            7354,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 536,
                                                            end: 538,
                                                            as_str(): "()",
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
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 513,
                                                                                        end: 553,
                                                                                        as_str(): "true {\n        true => (),\n        false",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 513,
                                                                                        end: 553,
                                                                                        as_str(): "true {\n        true => (),\n        false",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 513,
                                                                                    end: 553,
                                                                                    as_str(): "true {\n        true => (),\n        false",
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
                                                                                        src (ptr): 0x00007fe096926ee0,
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
                                                                                                "__match_return_var_name_5",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 513,
                                                                                                end: 517,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 513,
                                                                                            end: 517,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                        mutability: Immutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        71,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 513,
                                                                                        end: 517,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe096926ee0,
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
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 548,
                                                                                        end: 553,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            551,
                                                                            Span {
                                                                                src (ptr): 0x00007fe096926ee0,
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 513,
                                                                        end: 553,
                                                                        as_str(): "true {\n        true => (),\n        false",
                                                                    },
                                                                },
                                                                then: TyExpression {
                                                                    expression: CodeBlock(
                                                                        TyCodeBlock {
                                                                            contents: [
                                                                                TyAstNode {
                                                                                    content: ImplicitReturnExpression(
                                                                                        TyExpression {
                                                                                            expression: Tuple {
                                                                                                fields: [],
                                                                                            },
                                                                                            return_type: TypeId(
                                                                                                7357,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 557,
                                                                                                end: 559,
                                                                                                as_str(): "()",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 557,
                                                                                        end: 559,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        7357,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 557,
                                                                        end: 559,
                                                                        as_str(): "()",
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
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 569,
                                                                                                            end: 572,
                                                                                                            as_str(): "foo",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    body: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "__match_return_var_name_5",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 513,
                                                                                                                    end: 517,
                                                                                                                    as_str(): "true",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 513,
                                                                                                                end: 517,
                                                                                                                as_str(): "true",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            71,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 513,
                                                                                                            end: 517,
                                                                                                            as_str(): "true",
                                                                                                        },
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                    return_type: TypeId(
                                                                                                        71,
                                                                                                    ),
                                                                                                    type_ascription: TypeId(
                                                                                                        71,
                                                                                                    ),
                                                                                                    type_ascription_span: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 569,
                                                                                            end: 572,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                    },
                                                                                    TyAstNode {
                                                                                        content: ImplicitReturnExpression(
                                                                                            TyExpression {
                                                                                                expression: Tuple {
                                                                                                    fields: [],
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7361,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 576,
                                                                                                    end: 578,
                                                                                                    as_str(): "()",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 576,
                                                                                            end: 578,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        return_type: TypeId(
                                                                            7361,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 576,
                                                                            end: 578,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            return_type: TypeId(
                                                                7357,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 557,
                                                                end: 559,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    7362,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 536,
                                                    end: 538,
                                                    as_str(): "()",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                            ),
                                            start: 507,
                                            end: 623,
                                            as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
                                        },
                                    },
                                ],
                            },
                        ),
                        return_type: TypeId(
                            7363,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09ea4f9c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                            ),
                            start: 507,
                            end: 623,
                            as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09ea4f9c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                    ),
                    start: 507,
                    end: 623,
                    as_str(): "match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
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
                                        expression: LazyOperator {
                                            op: And,
                                            lhs: TyExpression {
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 634,
                                                                            end: 636,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 634,
                                                                            end: 636,
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 634,
                                                                        end: 636,
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
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 51,
                                                                                    end: 52,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 632,
                                                                                end: 633,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            7254,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 632,
                                                                            end: 633,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                                6,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 637,
                                                                            end: 638,
                                                                            as_str(): "6",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                553,
                                                                Span {
                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 634,
                                                                        end: 636,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 632,
                                                            end: 638,
                                                            as_str(): "b == 6",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 644,
                                                                            end: 646,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 644,
                                                                            end: 646,
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 644,
                                                                        end: 646,
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
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 162,
                                                                                    end: 163,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 642,
                                                                                end: 643,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            7271,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 642,
                                                                            end: 643,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 647,
                                                                            end: 649,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                554,
                                                                Span {
                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 644,
                                                                        end: 646,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 642,
                                                            end: 649,
                                                            as_str(): "c == 42",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 632,
                                                    end: 649,
                                                    as_str(): "b == 6 && c == 42",
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 655,
                                                                    end: 657,
                                                                    as_str(): "==",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 655,
                                                                    end: 657,
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
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 655,
                                                                end: 657,
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
                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 233,
                                                                            end: 234,
                                                                            as_str(): "d",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 653,
                                                                        end: 654,
                                                                        as_str(): "d",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7281,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 653,
                                                                    end: 654,
                                                                    as_str(): "d",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096926ee0,
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
                                                                        42,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 658,
                                                                    end: 660,
                                                                    as_str(): "42",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        555,
                                                        Span {
                                                            src (ptr): 0x00007fe096926ee0,
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
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 655,
                                                                end: 657,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 653,
                                                    end: 660,
                                                    as_str(): "d == 42",
                                                },
                                            },
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                            ),
                                            start: 632,
                                            end: 660,
                                            as_str(): "b == 6 && c == 42 && d == 42",
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 666,
                                                            end: 668,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 666,
                                                            end: 668,
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
                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                        ),
                                                        start: 666,
                                                        end: 668,
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
                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 399,
                                                                    end: 400,
                                                                    as_str(): "f",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 664,
                                                                end: 665,
                                                                as_str(): "f",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            7316,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 664,
                                                            end: 665,
                                                            as_str(): "f",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe096926ee0,
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
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 669,
                                                            end: 671,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                556,
                                                Span {
                                                    src (ptr): 0x00007fe096926ee0,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                        ),
                                                        start: 666,
                                                        end: 668,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                            ),
                                            start: 664,
                                            end: 671,
                                            as_str(): "f == 42",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 632,
                                    end: 671,
                                    as_str(): "b == 6 && c == 42 && d == 42 && f == 42",
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 682,
                                                            end: 684,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                    ),
                                                    start: 682,
                                                    end: 684,
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
                                    src (ptr): 0x00007fe09ea4f9c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                    ),
                                    start: 672,
                                    end: 690,
                                    as_str(): "{\n        42\n    }",
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
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 706,
                                                                end: 707,
                                                                as_str(): "0",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                        ),
                                                        start: 706,
                                                        end: 707,
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
                                        src (ptr): 0x00007fe09ea4f9c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                        ),
                                        start: 696,
                                        end: 713,
                                        as_str(): "{\n        0\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe09ea4f9c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                            ),
                            start: 629,
                            end: 713,
                            as_str(): "if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe09ea4f9c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                    ),
                    start: 629,
                    end: 713,
                    as_str(): "if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe09ea4f9c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
        ),
        start: 9,
        end: 715,
        as_str(): "fn main() -> u64 {\n    let a = 5;\n    let b = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let c = match a {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    let d = match 42 {\n        0 => { 24 },\n        foo => { foo },\n    };\n    let e = (\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    );\n    let f = match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    };\n\n    match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }\n\n    if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }\n}",
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
        src (ptr): 0x00007fe09ea4f9c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
        ),
        start: 22,
        end: 25,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

