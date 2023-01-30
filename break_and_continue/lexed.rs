Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb12b9f78e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb12b9f78e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Group {
                                        imports: Braces {
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 19,
                                                                    end: 25,
                                                                    as_str(): "assert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 25,
                                                                    end: 27,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 27,
                                                                        end: 33,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 33,
                                                                end: 34,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    Path {
                                                        prefix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 42,
                                                                as_str(): "logging",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 42,
                                                                end: 44,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 44,
                                                                    end: 47,
                                                                    as_str(): "log",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                ),
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb12b9f78e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 48,
                                                as_str(): "{assert::assert, logging::log}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 49,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 51,
                                        end: 56,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 57,
                                        end: 58,
                                        as_str(): "N",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 60,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fb12b9f78e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                ),
                                                start: 61,
                                                end: 63,
                                                as_str(): "10",
                                            },
                                            parsed: 10,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 63,
                                        end: 64,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 68,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 69,
                                            end: 86,
                                            as_str(): "simple_break_test",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 86,
                                            end: 88,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 95,
                                                            end: 98,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 99,
                                                                    end: 102,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 103,
                                                                end: 104,
                                                                as_str(): "i",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 105,
                                                            end: 106,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 107,
                                                                    end: 108,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 109,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 119,
                                                            as_str(): "while",
                                                        },
                                                    },
                                                    condition: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 120,
                                                                    end: 124,
                                                                    as_str(): "true",
                                                                },
                                                                kind: True,
                                                            },
                                                        ),
                                                    ),
                                                    block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: If(
                                                                        IfExpr {
                                                                            if_token: IfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 135,
                                                                                    end: 137,
                                                                                    as_str(): "if",
                                                                                },
                                                                            },
                                                                            condition: Expr(
                                                                                GreaterThanEq {
                                                                                    lhs: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 138,
                                                                                                        end: 139,
                                                                                                        as_str(): "i",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [],
                                                                                            incomplete_suffix: false,
                                                                                        },
                                                                                    ),
                                                                                    greater_than_eq_token: GreaterThanEqToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 140,
                                                                                            end: 142,
                                                                                            as_str(): ">=",
                                                                                        },
                                                                                    },
                                                                                    rhs: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 143,
                                                                                                        end: 144,
                                                                                                        as_str(): "N",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [],
                                                                                            incomplete_suffix: false,
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            then_block: Braces {
                                                                                inner: CodeBlockContents {
                                                                                    statements: [
                                                                                        Expr {
                                                                                            expr: Break {
                                                                                                break_token: BreakToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 159,
                                                                                                        end: 164,
                                                                                                        as_str(): "break",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                            semicolon_token_opt: Some(
                                                                                                SemicolonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 164,
                                                                                                        end: 165,
                                                                                                        as_str(): ";",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ],
                                                                                    final_expr_opt: None,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 145,
                                                                                    end: 175,
                                                                                    as_str(): "{\n            break;\n        }",
                                                                                },
                                                                            },
                                                                            else_opt: None,
                                                                        },
                                                                    ),
                                                                    semicolon_token_opt: None,
                                                                },
                                                            ],
                                                            final_expr_opt: Some(
                                                                Reassignment {
                                                                    assignable: Var(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 184,
                                                                                end: 185,
                                                                                as_str(): "i",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    reassignment_op: ReassignmentOp {
                                                                        variant: AddEquals,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 186,
                                                                            end: 188,
                                                                            as_str(): "+=",
                                                                        },
                                                                    },
                                                                    expr: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 189,
                                                                                    end: 190,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 125,
                                                            end: 196,
                                                            as_str(): "{\n        if i >= N {\n            break;\n        }\n        i += 1\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 201,
                                                                        end: 207,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Equal {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 208,
                                                                                        end: 209,
                                                                                        as_str(): "i",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 210,
                                                                            end: 212,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 213,
                                                                                        end: 214,
                                                                                        as_str(): "N",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 207,
                                                            end: 215,
                                                            as_str(): "(i == N)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 215,
                                                            end: 216,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 89,
                                        end: 218,
                                        as_str(): "{\n    let mut i = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        i += 1\n    }\n    assert(i == N);\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 220,
                                            end: 222,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 223,
                                            end: 243,
                                            as_str(): "simple_continue_test",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 243,
                                            end: 245,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 252,
                                                            end: 255,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 256,
                                                                    end: 259,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 260,
                                                                end: 261,
                                                                as_str(): "i",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 262,
                                                            end: 263,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 264,
                                                                    end: 265,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 266,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 271,
                                                            end: 274,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 275,
                                                                    end: 278,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 279,
                                                                end: 282,
                                                                as_str(): "sum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 283,
                                                            end: 284,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 285,
                                                                    end: 286,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 287,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 292,
                                                            end: 297,
                                                            as_str(): "while",
                                                        },
                                                    },
                                                    condition: LessThan {
                                                        lhs: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 298,
                                                                            end: 299,
                                                                            as_str(): "i",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                                incomplete_suffix: false,
                                                            },
                                                        ),
                                                        less_than_token: LessThanToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 300,
                                                                end: 301,
                                                                as_str(): "<",
                                                            },
                                                        },
                                                        rhs: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 302,
                                                                            end: 303,
                                                                            as_str(): "N",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                                incomplete_suffix: false,
                                                            },
                                                        ),
                                                    },
                                                    block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: Reassignment {
                                                                        assignable: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 314,
                                                                                    end: 315,
                                                                                    as_str(): "i",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: AddEquals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 316,
                                                                                end: 318,
                                                                                as_str(): "+=",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 319,
                                                                                        end: 320,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 320,
                                                                                end: 321,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                Expr {
                                                                    expr: If(
                                                                        IfExpr {
                                                                            if_token: IfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 330,
                                                                                    end: 332,
                                                                                    as_str(): "if",
                                                                                },
                                                                            },
                                                                            condition: Expr(
                                                                                Equal {
                                                                                    lhs: Modulo {
                                                                                        lhs: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 333,
                                                                                                            end: 334,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                                suffix: [],
                                                                                                incomplete_suffix: false,
                                                                                            },
                                                                                        ),
                                                                                        percent_token: PercentToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 335,
                                                                                                end: 336,
                                                                                                as_str(): "%",
                                                                                            },
                                                                                        },
                                                                                        rhs: Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 337,
                                                                                                        end: 338,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                    parsed: 2,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    double_eq_token: DoubleEqToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 339,
                                                                                            end: 341,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                    },
                                                                                    rhs: Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 342,
                                                                                                    end: 343,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            then_block: Braces {
                                                                                inner: CodeBlockContents {
                                                                                    statements: [
                                                                                        Expr {
                                                                                            expr: Continue {
                                                                                                continue_token: ContinueToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 358,
                                                                                                        end: 366,
                                                                                                        as_str(): "continue",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                            semicolon_token_opt: Some(
                                                                                                SemicolonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 366,
                                                                                                        end: 367,
                                                                                                        as_str(): ";",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ],
                                                                                    final_expr_opt: None,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 344,
                                                                                    end: 377,
                                                                                    as_str(): "{\n            continue;\n        }",
                                                                                },
                                                                            },
                                                                            else_opt: None,
                                                                        },
                                                                    ),
                                                                    semicolon_token_opt: None,
                                                                },
                                                                Expr {
                                                                    expr: Reassignment {
                                                                        assignable: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 386,
                                                                                    end: 389,
                                                                                    as_str(): "sum",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: AddEquals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 390,
                                                                                end: 392,
                                                                                as_str(): "+=",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 393,
                                                                                        end: 394,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 394,
                                                                                end: 395,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 304,
                                                            end: 401,
                                                            as_str(): "{\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 406,
                                                                        end: 412,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Equal {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 413,
                                                                                        end: 416,
                                                                                        as_str(): "sum",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 417,
                                                                            end: 419,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Div {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 420,
                                                                                            end: 421,
                                                                                            as_str(): "N",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        forward_slash_token: ForwardSlashToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 422,
                                                                                end: 423,
                                                                                as_str(): "/",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 424,
                                                                                        end: 425,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 412,
                                                            end: 426,
                                                            as_str(): "(sum == N / 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 426,
                                                            end: 427,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 246,
                                        end: 429,
                                        as_str(): "{\n    let mut i = 0;\n    let mut sum = 0;\n    while i < N {\n        i += 1;\n        if i % 2 == 0 {\n            continue;\n        }\n        sum += 1;\n    }\n    assert(sum == N / 2);\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 431,
                                            end: 433,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 434,
                                            end: 457,
                                            as_str(): "break_and_continue_test",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 457,
                                            end: 459,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 466,
                                                            end: 469,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 470,
                                                                    end: 473,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 474,
                                                                end: 475,
                                                                as_str(): "i",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 476,
                                                            end: 477,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 478,
                                                                    end: 479,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 479,
                                                            end: 480,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 485,
                                                            end: 488,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 489,
                                                                    end: 492,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 493,
                                                                end: 494,
                                                                as_str(): "j",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 495,
                                                            end: 496,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 497,
                                                                    end: 498,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 498,
                                                            end: 499,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 504,
                                                            end: 507,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 508,
                                                                    end: 511,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 512,
                                                                end: 513,
                                                                as_str(): "k",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 514,
                                                            end: 515,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 516,
                                                                    end: 517,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 517,
                                                            end: 518,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 523,
                                                            end: 526,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 527,
                                                                    end: 530,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 531,
                                                                end: 532,
                                                                as_str(): "n",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 533,
                                                            end: 534,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 535,
                                                                    end: 536,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 536,
                                                            end: 537,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 542,
                                                            end: 545,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 546,
                                                                    end: 549,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 550,
                                                                end: 554,
                                                                as_str(): "sum1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 555,
                                                            end: 556,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 557,
                                                                    end: 558,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 558,
                                                            end: 559,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 564,
                                                            end: 567,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 568,
                                                                    end: 571,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 572,
                                                                end: 576,
                                                                as_str(): "sum2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 577,
                                                            end: 578,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 579,
                                                                    end: 580,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 580,
                                                            end: 581,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 586,
                                                            end: 591,
                                                            as_str(): "while",
                                                        },
                                                    },
                                                    condition: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                    ),
                                                                    start: 592,
                                                                    end: 596,
                                                                    as_str(): "true",
                                                                },
                                                                kind: True,
                                                            },
                                                        ),
                                                    ),
                                                    block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: If(
                                                                        IfExpr {
                                                                            if_token: IfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 607,
                                                                                    end: 609,
                                                                                    as_str(): "if",
                                                                                },
                                                                            },
                                                                            condition: Expr(
                                                                                GreaterThanEq {
                                                                                    lhs: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 610,
                                                                                                        end: 611,
                                                                                                        as_str(): "i",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [],
                                                                                            incomplete_suffix: false,
                                                                                        },
                                                                                    ),
                                                                                    greater_than_eq_token: GreaterThanEqToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 612,
                                                                                            end: 614,
                                                                                            as_str(): ">=",
                                                                                        },
                                                                                    },
                                                                                    rhs: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 615,
                                                                                                        end: 616,
                                                                                                        as_str(): "N",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [],
                                                                                            incomplete_suffix: false,
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            then_block: Braces {
                                                                                inner: CodeBlockContents {
                                                                                    statements: [
                                                                                        Expr {
                                                                                            expr: Break {
                                                                                                break_token: BreakToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 631,
                                                                                                        end: 636,
                                                                                                        as_str(): "break",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                            semicolon_token_opt: Some(
                                                                                                SemicolonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 636,
                                                                                                        end: 637,
                                                                                                        as_str(): ";",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ],
                                                                                    final_expr_opt: None,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 617,
                                                                                    end: 647,
                                                                                    as_str(): "{\n            break;\n        }",
                                                                                },
                                                                            },
                                                                            else_opt: None,
                                                                        },
                                                                    ),
                                                                    semicolon_token_opt: None,
                                                                },
                                                                Expr {
                                                                    expr: While {
                                                                        while_token: WhileToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 656,
                                                                                end: 661,
                                                                                as_str(): "while",
                                                                            },
                                                                        },
                                                                        condition: Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 662,
                                                                                        end: 666,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [
                                                                                    Expr {
                                                                                        expr: If(
                                                                                            IfExpr {
                                                                                                if_token: IfToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 681,
                                                                                                        end: 683,
                                                                                                        as_str(): "if",
                                                                                                    },
                                                                                                },
                                                                                                condition: Expr(
                                                                                                    GreaterThanEq {
                                                                                                        lhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 684,
                                                                                                                            end: 685,
                                                                                                                            as_str(): "j",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                                incomplete_suffix: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        greater_than_eq_token: GreaterThanEqToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 686,
                                                                                                                end: 688,
                                                                                                                as_str(): ">=",
                                                                                                            },
                                                                                                        },
                                                                                                        rhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 689,
                                                                                                                            end: 690,
                                                                                                                            as_str(): "N",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                                incomplete_suffix: false,
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                                then_block: Braces {
                                                                                                    inner: CodeBlockContents {
                                                                                                        statements: [
                                                                                                            Expr {
                                                                                                                expr: Break {
                                                                                                                    break_token: BreakToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 709,
                                                                                                                            end: 714,
                                                                                                                            as_str(): "break",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                                semicolon_token_opt: Some(
                                                                                                                    SemicolonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 714,
                                                                                                                            end: 715,
                                                                                                                            as_str(): ";",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                        ],
                                                                                                        final_expr_opt: None,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 691,
                                                                                                        end: 729,
                                                                                                        as_str(): "{\n                break;\n            }",
                                                                                                    },
                                                                                                },
                                                                                                else_opt: None,
                                                                                            },
                                                                                        ),
                                                                                        semicolon_token_opt: None,
                                                                                    },
                                                                                    Expr {
                                                                                        expr: Reassignment {
                                                                                            assignable: Var(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 742,
                                                                                                        end: 746,
                                                                                                        as_str(): "sum1",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            reassignment_op: ReassignmentOp {
                                                                                                variant: AddEquals,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 747,
                                                                                                    end: 749,
                                                                                                    as_str(): "+=",
                                                                                                },
                                                                                            },
                                                                                            expr: Add {
                                                                                                lhs: Mul {
                                                                                                    lhs: Path(
                                                                                                        PathExpr {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 750,
                                                                                                                        end: 751,
                                                                                                                        as_str(): "i",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                generics_opt: None,
                                                                                                            },
                                                                                                            suffix: [],
                                                                                                            incomplete_suffix: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    star_token: StarToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 752,
                                                                                                            end: 753,
                                                                                                            as_str(): "*",
                                                                                                        },
                                                                                                    },
                                                                                                    rhs: Path(
                                                                                                        PathExpr {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 754,
                                                                                                                        end: 755,
                                                                                                                        as_str(): "N",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                generics_opt: None,
                                                                                                            },
                                                                                                            suffix: [],
                                                                                                            incomplete_suffix: false,
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                add_token: AddToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 756,
                                                                                                        end: 757,
                                                                                                        as_str(): "+",
                                                                                                    },
                                                                                                },
                                                                                                rhs: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 758,
                                                                                                                    end: 759,
                                                                                                                    as_str(): "j",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                        incomplete_suffix: false,
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 759,
                                                                                                    end: 760,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    Expr {
                                                                                        expr: Reassignment {
                                                                                            assignable: Var(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 773,
                                                                                                        end: 774,
                                                                                                        as_str(): "j",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            reassignment_op: ReassignmentOp {
                                                                                                variant: AddEquals,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 775,
                                                                                                    end: 777,
                                                                                                    as_str(): "+=",
                                                                                                },
                                                                                            },
                                                                                            expr: Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 778,
                                                                                                            end: 779,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                        parsed: 1,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 779,
                                                                                                    end: 780,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    Expr {
                                                                                        expr: If(
                                                                                            IfExpr {
                                                                                                if_token: IfToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 794,
                                                                                                        end: 796,
                                                                                                        as_str(): "if",
                                                                                                    },
                                                                                                },
                                                                                                condition: Expr(
                                                                                                    Equal {
                                                                                                        lhs: Modulo {
                                                                                                            lhs: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 797,
                                                                                                                                end: 798,
                                                                                                                                as_str(): "j",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [],
                                                                                                                    incomplete_suffix: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            percent_token: PercentToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 799,
                                                                                                                    end: 800,
                                                                                                                    as_str(): "%",
                                                                                                                },
                                                                                                            },
                                                                                                            rhs: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 801,
                                                                                                                            end: 802,
                                                                                                                            as_str(): "2",
                                                                                                                        },
                                                                                                                        parsed: 2,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        double_eq_token: DoubleEqToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 803,
                                                                                                                end: 805,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                        },
                                                                                                        rhs: Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 806,
                                                                                                                        end: 807,
                                                                                                                        as_str(): "0",
                                                                                                                    },
                                                                                                                    parsed: 0,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                                then_block: Braces {
                                                                                                    inner: CodeBlockContents {
                                                                                                        statements: [
                                                                                                            Expr {
                                                                                                                expr: Continue {
                                                                                                                    continue_token: ContinueToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 826,
                                                                                                                            end: 834,
                                                                                                                            as_str(): "continue",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                                semicolon_token_opt: Some(
                                                                                                                    SemicolonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 834,
                                                                                                                            end: 835,
                                                                                                                            as_str(): ";",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                        ],
                                                                                                        final_expr_opt: None,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 808,
                                                                                                        end: 849,
                                                                                                        as_str(): "{\n                continue;\n            }",
                                                                                                    },
                                                                                                },
                                                                                                else_opt: None,
                                                                                            },
                                                                                        ),
                                                                                        semicolon_token_opt: None,
                                                                                    },
                                                                                ],
                                                                                final_expr_opt: Some(
                                                                                    While {
                                                                                        while_token: WhileToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 863,
                                                                                                end: 868,
                                                                                                as_str(): "while",
                                                                                            },
                                                                                        },
                                                                                        condition: LessThan {
                                                                                            lhs: Path(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 869,
                                                                                                                end: 870,
                                                                                                                as_str(): "n",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                    suffix: [],
                                                                                                    incomplete_suffix: false,
                                                                                                },
                                                                                            ),
                                                                                            less_than_token: LessThanToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 871,
                                                                                                    end: 872,
                                                                                                    as_str(): "<",
                                                                                                },
                                                                                            },
                                                                                            rhs: Path(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 873,
                                                                                                                end: 874,
                                                                                                                as_str(): "N",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                    suffix: [],
                                                                                                    incomplete_suffix: false,
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        block: Braces {
                                                                                            inner: CodeBlockContents {
                                                                                                statements: [
                                                                                                    Expr {
                                                                                                        expr: Reassignment {
                                                                                                            assignable: Var(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 893,
                                                                                                                        end: 897,
                                                                                                                        as_str(): "sum1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            reassignment_op: ReassignmentOp {
                                                                                                                variant: AddEquals,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 898,
                                                                                                                    end: 900,
                                                                                                                    as_str(): "+=",
                                                                                                                },
                                                                                                            },
                                                                                                            expr: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 901,
                                                                                                                                end: 902,
                                                                                                                                as_str(): "n",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [],
                                                                                                                    incomplete_suffix: false,
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        semicolon_token_opt: Some(
                                                                                                            SemicolonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 902,
                                                                                                                    end: 903,
                                                                                                                    as_str(): ";",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    Expr {
                                                                                                        expr: Reassignment {
                                                                                                            assignable: Var(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 920,
                                                                                                                        end: 921,
                                                                                                                        as_str(): "n",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            reassignment_op: ReassignmentOp {
                                                                                                                variant: AddEquals,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 922,
                                                                                                                    end: 924,
                                                                                                                    as_str(): "+=",
                                                                                                                },
                                                                                                            },
                                                                                                            expr: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 925,
                                                                                                                            end: 926,
                                                                                                                            as_str(): "1",
                                                                                                                        },
                                                                                                                        parsed: 1,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        semicolon_token_opt: Some(
                                                                                                            SemicolonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 926,
                                                                                                                    end: 927,
                                                                                                                    as_str(): ";",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                ],
                                                                                                final_expr_opt: Some(
                                                                                                    If(
                                                                                                        IfExpr {
                                                                                                            if_token: IfToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 944,
                                                                                                                    end: 946,
                                                                                                                    as_str(): "if",
                                                                                                                },
                                                                                                            },
                                                                                                            condition: Expr(
                                                                                                                GreaterThan {
                                                                                                                    lhs: Path(
                                                                                                                        PathExpr {
                                                                                                                            root_opt: None,
                                                                                                                            prefix: PathExprSegment {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 947,
                                                                                                                                        end: 951,
                                                                                                                                        as_str(): "sum1",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                generics_opt: None,
                                                                                                                            },
                                                                                                                            suffix: [],
                                                                                                                            incomplete_suffix: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    greater_than_token: GreaterThanToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 952,
                                                                                                                            end: 953,
                                                                                                                            as_str(): ">",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    rhs: Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 954,
                                                                                                                                    end: 956,
                                                                                                                                    as_str(): "50",
                                                                                                                                },
                                                                                                                                parsed: 50,
                                                                                                                                ty_opt: None,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                },
                                                                                                            ),
                                                                                                            then_block: Braces {
                                                                                                                inner: CodeBlockContents {
                                                                                                                    statements: [
                                                                                                                        Expr {
                                                                                                                            expr: Break {
                                                                                                                                break_token: BreakToken {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 979,
                                                                                                                                        end: 984,
                                                                                                                                        as_str(): "break",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            },
                                                                                                                            semicolon_token_opt: Some(
                                                                                                                                SemicolonToken {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 984,
                                                                                                                                        end: 985,
                                                                                                                                        as_str(): ";",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    final_expr_opt: None,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 957,
                                                                                                                    end: 1003,
                                                                                                                    as_str(): "{\n                    break;\n                }",
                                                                                                                },
                                                                                                            },
                                                                                                            else_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 875,
                                                                                                end: 1017,
                                                                                                as_str(): "{\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 667,
                                                                                end: 1027,
                                                                                as_str(): "{\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }",
                                                                            },
                                                                        },
                                                                    },
                                                                    semicolon_token_opt: None,
                                                                },
                                                                Expr {
                                                                    expr: While {
                                                                        while_token: WhileToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1037,
                                                                                end: 1042,
                                                                                as_str(): "while",
                                                                            },
                                                                        },
                                                                        condition: Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1043,
                                                                                        end: 1047,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [
                                                                                    Expr {
                                                                                        expr: If(
                                                                                            IfExpr {
                                                                                                if_token: IfToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1062,
                                                                                                        end: 1064,
                                                                                                        as_str(): "if",
                                                                                                    },
                                                                                                },
                                                                                                condition: Expr(
                                                                                                    GreaterThanEq {
                                                                                                        lhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1065,
                                                                                                                            end: 1066,
                                                                                                                            as_str(): "k",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                                incomplete_suffix: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        greater_than_eq_token: GreaterThanEqToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1067,
                                                                                                                end: 1069,
                                                                                                                as_str(): ">=",
                                                                                                            },
                                                                                                        },
                                                                                                        rhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1070,
                                                                                                                            end: 1071,
                                                                                                                            as_str(): "N",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                                incomplete_suffix: false,
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                                then_block: Braces {
                                                                                                    inner: CodeBlockContents {
                                                                                                        statements: [
                                                                                                            Expr {
                                                                                                                expr: Break {
                                                                                                                    break_token: BreakToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1090,
                                                                                                                            end: 1095,
                                                                                                                            as_str(): "break",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                                semicolon_token_opt: Some(
                                                                                                                    SemicolonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1095,
                                                                                                                            end: 1096,
                                                                                                                            as_str(): ";",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                        ],
                                                                                                        final_expr_opt: None,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1072,
                                                                                                        end: 1110,
                                                                                                        as_str(): "{\n                break;\n            }",
                                                                                                    },
                                                                                                },
                                                                                                else_opt: None,
                                                                                            },
                                                                                        ),
                                                                                        semicolon_token_opt: None,
                                                                                    },
                                                                                    Expr {
                                                                                        expr: Reassignment {
                                                                                            assignable: Var(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1123,
                                                                                                        end: 1127,
                                                                                                        as_str(): "sum1",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            reassignment_op: ReassignmentOp {
                                                                                                variant: AddEquals,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1128,
                                                                                                    end: 1130,
                                                                                                    as_str(): "+=",
                                                                                                },
                                                                                            },
                                                                                            expr: Add {
                                                                                                lhs: Mul {
                                                                                                    lhs: Path(
                                                                                                        PathExpr {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1131,
                                                                                                                        end: 1132,
                                                                                                                        as_str(): "i",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                generics_opt: None,
                                                                                                            },
                                                                                                            suffix: [],
                                                                                                            incomplete_suffix: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    star_token: StarToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1133,
                                                                                                            end: 1134,
                                                                                                            as_str(): "*",
                                                                                                        },
                                                                                                    },
                                                                                                    rhs: Path(
                                                                                                        PathExpr {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1135,
                                                                                                                        end: 1136,
                                                                                                                        as_str(): "N",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                generics_opt: None,
                                                                                                            },
                                                                                                            suffix: [],
                                                                                                            incomplete_suffix: false,
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                add_token: AddToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1137,
                                                                                                        end: 1138,
                                                                                                        as_str(): "+",
                                                                                                    },
                                                                                                },
                                                                                                rhs: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1139,
                                                                                                                    end: 1140,
                                                                                                                    as_str(): "k",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                        incomplete_suffix: false,
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1140,
                                                                                                    end: 1141,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    Expr {
                                                                                        expr: Reassignment {
                                                                                            assignable: Var(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1154,
                                                                                                        end: 1155,
                                                                                                        as_str(): "k",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            reassignment_op: ReassignmentOp {
                                                                                                variant: AddEquals,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1156,
                                                                                                    end: 1158,
                                                                                                    as_str(): "+=",
                                                                                                },
                                                                                            },
                                                                                            expr: Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1159,
                                                                                                            end: 1160,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                        parsed: 1,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1160,
                                                                                                    end: 1161,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    Expr {
                                                                                        expr: If(
                                                                                            IfExpr {
                                                                                                if_token: IfToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1175,
                                                                                                        end: 1177,
                                                                                                        as_str(): "if",
                                                                                                    },
                                                                                                },
                                                                                                condition: Expr(
                                                                                                    Equal {
                                                                                                        lhs: Modulo {
                                                                                                            lhs: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1178,
                                                                                                                                end: 1179,
                                                                                                                                as_str(): "k",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [],
                                                                                                                    incomplete_suffix: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            percent_token: PercentToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1180,
                                                                                                                    end: 1181,
                                                                                                                    as_str(): "%",
                                                                                                                },
                                                                                                            },
                                                                                                            rhs: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1182,
                                                                                                                            end: 1183,
                                                                                                                            as_str(): "2",
                                                                                                                        },
                                                                                                                        parsed: 2,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        double_eq_token: DoubleEqToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1184,
                                                                                                                end: 1186,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                        },
                                                                                                        rhs: Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1187,
                                                                                                                        end: 1188,
                                                                                                                        as_str(): "0",
                                                                                                                    },
                                                                                                                    parsed: 0,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                                then_block: Braces {
                                                                                                    inner: CodeBlockContents {
                                                                                                        statements: [
                                                                                                            Expr {
                                                                                                                expr: Continue {
                                                                                                                    continue_token: ContinueToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1207,
                                                                                                                            end: 1215,
                                                                                                                            as_str(): "continue",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                                semicolon_token_opt: Some(
                                                                                                                    SemicolonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1215,
                                                                                                                            end: 1216,
                                                                                                                            as_str(): ";",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                        ],
                                                                                                        final_expr_opt: None,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1189,
                                                                                                        end: 1230,
                                                                                                        as_str(): "{\n                continue;\n            }",
                                                                                                    },
                                                                                                },
                                                                                                else_opt: None,
                                                                                            },
                                                                                        ),
                                                                                        semicolon_token_opt: None,
                                                                                    },
                                                                                    Expr {
                                                                                        expr: Reassignment {
                                                                                            assignable: Var(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1244,
                                                                                                        end: 1248,
                                                                                                        as_str(): "sum1",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            reassignment_op: ReassignmentOp {
                                                                                                variant: MulEquals,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1249,
                                                                                                    end: 1251,
                                                                                                    as_str(): "*=",
                                                                                                },
                                                                                            },
                                                                                            expr: Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1252,
                                                                                                            end: 1253,
                                                                                                            as_str(): "2",
                                                                                                        },
                                                                                                        parsed: 2,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1253,
                                                                                                    end: 1254,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ],
                                                                                final_expr_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1048,
                                                                                end: 1264,
                                                                                as_str(): "{\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }",
                                                                            },
                                                                        },
                                                                    },
                                                                    semicolon_token_opt: None,
                                                                },
                                                                Expr {
                                                                    expr: Reassignment {
                                                                        assignable: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1273,
                                                                                    end: 1274,
                                                                                    as_str(): "i",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: AddEquals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1275,
                                                                                end: 1277,
                                                                                as_str(): "+=",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1278,
                                                                                        end: 1279,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1279,
                                                                                end: 1280,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                Expr {
                                                                    expr: If(
                                                                        IfExpr {
                                                                            if_token: IfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1290,
                                                                                    end: 1292,
                                                                                    as_str(): "if",
                                                                                },
                                                                            },
                                                                            condition: Expr(
                                                                                Equal {
                                                                                    lhs: Modulo {
                                                                                        lhs: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1293,
                                                                                                            end: 1294,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                                suffix: [],
                                                                                                incomplete_suffix: false,
                                                                                            },
                                                                                        ),
                                                                                        percent_token: PercentToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                ),
                                                                                                start: 1295,
                                                                                                end: 1296,
                                                                                                as_str(): "%",
                                                                                            },
                                                                                        },
                                                                                        rhs: Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1297,
                                                                                                        end: 1298,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                    parsed: 2,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    double_eq_token: DoubleEqToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                            ),
                                                                                            start: 1299,
                                                                                            end: 1301,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                    },
                                                                                    rhs: Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1302,
                                                                                                    end: 1303,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            then_block: Braces {
                                                                                inner: CodeBlockContents {
                                                                                    statements: [
                                                                                        Expr {
                                                                                            expr: Continue {
                                                                                                continue_token: ContinueToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1318,
                                                                                                        end: 1326,
                                                                                                        as_str(): "continue",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                            semicolon_token_opt: Some(
                                                                                                SemicolonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1326,
                                                                                                        end: 1327,
                                                                                                        as_str(): ";",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ],
                                                                                    final_expr_opt: None,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1304,
                                                                                    end: 1337,
                                                                                    as_str(): "{\n            continue;\n        }",
                                                                                },
                                                                            },
                                                                            else_opt: None,
                                                                        },
                                                                    ),
                                                                    semicolon_token_opt: None,
                                                                },
                                                                Expr {
                                                                    expr: Reassignment {
                                                                        assignable: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1347,
                                                                                    end: 1351,
                                                                                    as_str(): "sum1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: AddEquals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1352,
                                                                                end: 1354,
                                                                                as_str(): "+=",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1355,
                                                                                        end: 1356,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1356,
                                                                                end: 1357,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                Expr {
                                                                    expr: Reassignment {
                                                                        assignable: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1366,
                                                                                    end: 1370,
                                                                                    as_str(): "sum2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: AddEquals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1371,
                                                                                end: 1373,
                                                                                as_str(): "+=",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1374,
                                                                                        end: 1375,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                ),
                                                                                start: 1375,
                                                                                end: 1376,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 597,
                                                            end: 1382,
                                                            as_str(): "{\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1388,
                                                                        end: 1394,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Equal {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1395,
                                                                                        end: 1399,
                                                                                        as_str(): "sum1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1400,
                                                                            end: 1402,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1403,
                                                                                    end: 1407,
                                                                                    as_str(): "3072",
                                                                                },
                                                                                parsed: 3072,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1394,
                                                            end: 1408,
                                                            as_str(): "(sum1 == 3072)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1408,
                                                            end: 1409,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1414,
                                                                        end: 1420,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Equal {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                        ),
                                                                                        start: 1421,
                                                                                        end: 1425,
                                                                                        as_str(): "sum2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12b9f78e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                            ),
                                                                            start: 1426,
                                                                            end: 1428,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12b9f78e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                                    ),
                                                                                    start: 1429,
                                                                                    end: 1430,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1420,
                                                            end: 1431,
                                                            as_str(): "(sum2 == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1431,
                                                            end: 1432,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 460,
                                        end: 1434,
                                        as_str(): "{\n    let mut i = 0;\n    let mut j = 0;\n    let mut k = 0;\n    let mut n = 0;\n    let mut sum1 = 0;\n    let mut sum2 = 0;\n    while true {\n        if i >= N {\n            break;\n        }\n        while true {\n            if j >= N {\n                break;\n            }\n            sum1 += i * N + j;\n            j += 1;\n\n            if j % 2 == 0 {\n                continue;\n            }\n\n            while n < N {\n                sum1 += n;\n                n += 1;\n                if sum1 > 50 {\n                    break;\n                }\n            }\n        }\n\n        while true {\n            if k >= N {\n                break;\n            }\n            sum1 += i * N + k;\n            k += 1;\n\n            if k % 2 == 0 {\n                continue;\n            }\n\n            sum1 *= 2;\n        }\n        i += 1;\n\n        if i % 2 == 0 {\n            continue;\n        }\n\n        sum1 += 1;\n        sum2 += 1;\n    }\n\n    assert(sum1 == 3072);\n    assert(sum2 == 5);\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1436,
                                            end: 1438,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1439,
                                            end: 1443,
                                            as_str(): "main",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12b9f78e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                            ),
                                            start: 1443,
                                            end: 1445,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb12b9f78e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                    ),
                                                    start: 1446,
                                                    end: 1448,
                                                    as_str(): "->",
                                                },
                                            },
                                            Path(
                                                PathType {
                                                    root_opt: None,
                                                    prefix: PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12b9f78e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                ),
                                                                start: 1449,
                                                                end: 1453,
                                                                as_str(): "bool",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    suffix: [],
                                                },
                                            ),
                                        ),
                                    ),
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1460,
                                                                        end: 1477,
                                                                        as_str(): "simple_break_test",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1477,
                                                            end: 1479,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1479,
                                                            end: 1480,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1485,
                                                                        end: 1505,
                                                                        as_str(): "simple_continue_test",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1505,
                                                            end: 1507,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1507,
                                                            end: 1508,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12b9f78e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                                        ),
                                                                        start: 1513,
                                                                        end: 1536,
                                                                        as_str(): "break_and_continue_test",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1536,
                                                            end: 1538,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1538,
                                                            end: 1539,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12b9f78e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                                            ),
                                                            start: 1545,
                                                            end: 1549,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb12b9f78e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMKIvI1/break_and_continue/src/main.sw",
                                        ),
                                        start: 1454,
                                        end: 1551,
                                        as_str(): "{\n    simple_break_test();\n    simple_continue_test();\n    break_and_continue_test();\n\n    true\n}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [],
        },
    },
)
