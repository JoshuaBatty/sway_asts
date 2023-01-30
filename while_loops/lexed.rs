Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a0e6287a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a0e6287a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
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
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007f8a0e6287a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007f8a0e6287a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                ),
                                                start: 24,
                                                end: 26,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 26,
                                                    end: 32,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 32,
                                        end: 33,
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
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 38,
                                            end: 42,
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
                                            src (ptr): 0x00007f8a0e6287a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                            ),
                                            start: 42,
                                            end: 44,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6287a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 47,
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 48,
                                                                end: 52,
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
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 62,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 66,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 67,
                                                                end: 74,
                                                                as_str(): "counter",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 76,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 77,
                                                                    end: 78,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 78,
                                                            end: 79,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 117,
                                                            end: 122,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 123,
                                                                            end: 130,
                                                                            as_str(): "counter",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 131,
                                                                end: 132,
                                                                as_str(): "<",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 133,
                                                                        end: 135,
                                                                        as_str(): "10",
                                                                    },
                                                                    parsed: 10,
                                                                    ty_opt: None,
                                                                },
                                                            ),
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 146,
                                                                                    end: 153,
                                                                                    as_str(): "counter",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: Equals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 154,
                                                                                end: 155,
                                                                                as_str(): "=",
                                                                            },
                                                                        },
                                                                        expr: Add {
                                                                            lhs: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 156,
                                                                                                end: 163,
                                                                                                as_str(): "counter",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [],
                                                                                    incomplete_suffix: false,
                                                                                },
                                                                            ),
                                                                            add_token: AddToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 164,
                                                                                    end: 165,
                                                                                    as_str(): "+",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                            ),
                                                                                            start: 166,
                                                                                            end: 167,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 167,
                                                                                end: 168,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 136,
                                                            end: 174,
                                                            as_str(): "{\n        counter = counter + 1;\n    }",
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
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 179,
                                                                        end: 185,
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 186,
                                                                                        end: 193,
                                                                                        as_str(): "counter",
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 196,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 197,
                                                                                    end: 199,
                                                                                    as_str(): "10",
                                                                                },
                                                                                parsed: 10,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 185,
                                                            end: 200,
                                                            as_str(): "(counter == 10)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 200,
                                                            end: 201,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 293,
                                                            end: 296,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 297,
                                                                    end: 300,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 301,
                                                                end: 310,
                                                                as_str(): "counter_2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 311,
                                                            end: 312,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 313,
                                                                    end: 314,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 314,
                                                            end: 315,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 320,
                                                            end: 323,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 324,
                                                                    end: 327,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 328,
                                                                end: 337,
                                                                as_str(): "counter_3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 338,
                                                            end: 339,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 340,
                                                                    end: 341,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 341,
                                                            end: 342,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 352,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 353,
                                                                            end: 362,
                                                                            as_str(): "counter_2",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 363,
                                                                end: 364,
                                                                as_str(): "<",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 365,
                                                                        end: 367,
                                                                        as_str(): "10",
                                                                    },
                                                                    parsed: 10,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                If(
                                                                    IfExpr {
                                                                        if_token: IfToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 378,
                                                                                end: 380,
                                                                                as_str(): "if",
                                                                            },
                                                                        },
                                                                        condition: Expr(
                                                                            Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 381,
                                                                                                    end: 390,
                                                                                                    as_str(): "counter_2",
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 391,
                                                                                        end: 393,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                rhs: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 394,
                                                                                                end: 395,
                                                                                                as_str(): "3",
                                                                                            },
                                                                                            parsed: 3,
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
                                                                                        expr: Reassignment {
                                                                                            assignable: Var(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 463,
                                                                                                        end: 472,
                                                                                                        as_str(): "counter_2",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            reassignment_op: ReassignmentOp {
                                                                                                variant: Equals,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 473,
                                                                                                    end: 474,
                                                                                                    as_str(): "=",
                                                                                                },
                                                                                            },
                                                                                            expr: Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 475,
                                                                                                            end: 477,
                                                                                                            as_str(): "10",
                                                                                                        },
                                                                                                        parsed: 10,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 477,
                                                                                                    end: 478,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ],
                                                                                final_expr_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 396,
                                                                                end: 488,
                                                                                as_str(): "{\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        }",
                                                                            },
                                                                        },
                                                                        else_opt: Some(
                                                                            (
                                                                                ElseToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 489,
                                                                                        end: 493,
                                                                                        as_str(): "else",
                                                                                    },
                                                                                },
                                                                                Break(
                                                                                    Braces {
                                                                                        inner: CodeBlockContents {
                                                                                            statements: [
                                                                                                Expr {
                                                                                                    expr: Reassignment {
                                                                                                        assignable: Var(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 508,
                                                                                                                    end: 517,
                                                                                                                    as_str(): "counter_2",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        reassignment_op: ReassignmentOp {
                                                                                                            variant: Equals,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 518,
                                                                                                                end: 519,
                                                                                                                as_str(): "=",
                                                                                                            },
                                                                                                        },
                                                                                                        expr: Add {
                                                                                                            lhs: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 520,
                                                                                                                                end: 529,
                                                                                                                                as_str(): "counter_2",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [],
                                                                                                                    incomplete_suffix: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            add_token: AddToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 530,
                                                                                                                    end: 531,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                            },
                                                                                                            rhs: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 532,
                                                                                                                            end: 533,
                                                                                                                            as_str(): "1",
                                                                                                                        },
                                                                                                                        parsed: 1,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                    },
                                                                                                    semicolon_token_opt: Some(
                                                                                                        SemicolonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 533,
                                                                                                                end: 534,
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
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 547,
                                                                                                                    end: 556,
                                                                                                                    as_str(): "counter_3",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        reassignment_op: ReassignmentOp {
                                                                                                            variant: Equals,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 557,
                                                                                                                end: 558,
                                                                                                                as_str(): "=",
                                                                                                            },
                                                                                                        },
                                                                                                        expr: Add {
                                                                                                            lhs: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 559,
                                                                                                                                end: 568,
                                                                                                                                as_str(): "counter_3",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [],
                                                                                                                    incomplete_suffix: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            add_token: AddToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 569,
                                                                                                                    end: 570,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                            },
                                                                                                            rhs: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 571,
                                                                                                                            end: 572,
                                                                                                                            as_str(): "1",
                                                                                                                        },
                                                                                                                        parsed: 1,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                    },
                                                                                                    semicolon_token_opt: Some(
                                                                                                        SemicolonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 572,
                                                                                                                end: 573,
                                                                                                                as_str(): ";",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                            ],
                                                                                            final_expr_opt: None,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                            ),
                                                                                            start: 494,
                                                                                            end: 583,
                                                                                            as_str(): "{\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 368,
                                                            end: 589,
                                                            as_str(): "{\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }",
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
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 595,
                                                                        end: 601,
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
                                                                LogicalAnd {
                                                                    lhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                            ),
                                                                                            start: 602,
                                                                                            end: 611,
                                                                                            as_str(): "counter_2",
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
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 612,
                                                                                end: 614,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 615,
                                                                                        end: 617,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                    parsed: 10,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 618,
                                                                            end: 620,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                            ),
                                                                                            start: 621,
                                                                                            end: 630,
                                                                                            as_str(): "counter_3",
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
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 631,
                                                                                end: 633,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 634,
                                                                                        end: 635,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 601,
                                                            end: 636,
                                                            as_str(): "(counter_2 == 10 && counter_3 == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 636,
                                                            end: 637,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 669,
                                                            end: 672,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 673,
                                                                    end: 676,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 677,
                                                                end: 686,
                                                                as_str(): "counter_4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 687,
                                                            end: 688,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 689,
                                                                    end: 690,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 690,
                                                            end: 691,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 696,
                                                            end: 699,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 700,
                                                                    end: 703,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 704,
                                                                end: 713,
                                                                as_str(): "counter_5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 714,
                                                            end: 715,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                    ),
                                                                    start: 716,
                                                                    end: 717,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 717,
                                                            end: 718,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 724,
                                                            end: 729,
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 730,
                                                                            end: 739,
                                                                            as_str(): "counter_4",
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
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 740,
                                                                end: 741,
                                                                as_str(): "<",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 742,
                                                                        end: 743,
                                                                        as_str(): "7",
                                                                    },
                                                                    parsed: 7,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: While {
                                                                        while_token: WhileToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 754,
                                                                                end: 759,
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
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 760,
                                                                                                end: 769,
                                                                                                as_str(): "counter_5",
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 770,
                                                                                    end: 771,
                                                                                    as_str(): "<",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                            ),
                                                                                            start: 772,
                                                                                            end: 774,
                                                                                            as_str(): "11",
                                                                                        },
                                                                                        parsed: 11,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
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
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 789,
                                                                                                        end: 798,
                                                                                                        as_str(): "counter_5",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            reassignment_op: ReassignmentOp {
                                                                                                variant: Equals,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 799,
                                                                                                    end: 800,
                                                                                                    as_str(): "=",
                                                                                                },
                                                                                            },
                                                                                            expr: Add {
                                                                                                lhs: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 801,
                                                                                                                    end: 810,
                                                                                                                    as_str(): "counter_5",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                        incomplete_suffix: false,
                                                                                                    },
                                                                                                ),
                                                                                                add_token: AddToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 811,
                                                                                                        end: 812,
                                                                                                        as_str(): "+",
                                                                                                    },
                                                                                                },
                                                                                                rhs: Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 813,
                                                                                                                end: 814,
                                                                                                                as_str(): "1",
                                                                                                            },
                                                                                                            parsed: 1,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 814,
                                                                                                    end: 815,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ],
                                                                                final_expr_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 775,
                                                                                end: 825,
                                                                                as_str(): "{\n            counter_5 = counter_5 + 1;\n        }",
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
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 834,
                                                                                    end: 843,
                                                                                    as_str(): "counter_4",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: Equals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 844,
                                                                                end: 845,
                                                                                as_str(): "=",
                                                                            },
                                                                        },
                                                                        expr: Add {
                                                                            lhs: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                                ),
                                                                                                start: 846,
                                                                                                end: 855,
                                                                                                as_str(): "counter_4",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [],
                                                                                    incomplete_suffix: false,
                                                                                },
                                                                            ),
                                                                            add_token: AddToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 856,
                                                                                    end: 857,
                                                                                    as_str(): "+",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                            ),
                                                                                            start: 858,
                                                                                            end: 859,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                ),
                                                                                start: 859,
                                                                                end: 860,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 744,
                                                            end: 866,
                                                            as_str(): "{\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }",
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
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 871,
                                                                        end: 877,
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 878,
                                                                                        end: 887,
                                                                                        as_str(): "counter_5",
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 888,
                                                                            end: 890,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 891,
                                                                                    end: 893,
                                                                                    as_str(): "11",
                                                                                },
                                                                                parsed: 11,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 877,
                                                            end: 894,
                                                            as_str(): "(counter_5 == 11)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 894,
                                                            end: 895,
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
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 900,
                                                                        end: 906,
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
                                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                        ),
                                                                                        start: 907,
                                                                                        end: 916,
                                                                                        as_str(): "counter_4",
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
                                                                            src (ptr): 0x00007f8a0e6287a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                            ),
                                                                            start: 917,
                                                                            end: 919,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 920,
                                                                                    end: 921,
                                                                                    as_str(): "7",
                                                                                },
                                                                                parsed: 7,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 906,
                                                            end: 922,
                                                            as_str(): "(counter_4 == 7)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 922,
                                                            end: 923,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 963,
                                                            end: 966,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 967,
                                                                end: 973,
                                                                as_str(): "result",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 974,
                                                            end: 975,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: While {
                                                        while_token: WhileToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 976,
                                                                end: 981,
                                                                as_str(): "while",
                                                            },
                                                        },
                                                        condition: Literal(
                                                            Bool(
                                                                LitBool {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a0e6287a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                        ),
                                                                        start: 982,
                                                                        end: 986,
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
                                                                        expr: Break {
                                                                            break_token: BreakToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 989,
                                                                                    end: 994,
                                                                                    as_str(): "break",
                                                                                },
                                                                            },
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a0e6287a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                                    ),
                                                                                    start: 994,
                                                                                    end: 995,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6287a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                                ),
                                                                start: 987,
                                                                end: 997,
                                                                as_str(): "{ break; }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 997,
                                                            end: 998,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6287a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                                            ),
                                                            start: 1004,
                                                            end: 1008,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a0e6287a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR97aUwT/while_loops/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 1010,
                                        as_str(): "{\n    let mut counter = 0;\n    // test standard while loop:\n    while counter < 10 {\n        counter = counter + 1;\n    }\n    assert(counter == 10);\n\n    // test early exit from loop with manual \"break\" (by invalidating the condition):\n    let mut counter_2 = 0;\n    let mut counter_3 = 0;\n    while counter_2 < 10 {\n        if counter_2 == 3 {\n            // ensure that condition is now invalid:\n            counter_2 = 10;\n        } else {\n            counter_2 = counter_2 + 1;\n            counter_3 = counter_3 + 1;\n        }\n    }\n\n    assert(counter_2 == 10 && counter_3 == 3);\n\n    // test nested loops:\n    let mut counter_4 = 0;\n    let mut counter_5 = 0;\n\n    while counter_4 < 7 {\n        while counter_5 < 11 {\n            counter_5 = counter_5 + 1;\n        }\n        counter_4 = counter_4 + 1;\n    }\n    assert(counter_5 == 11);\n    assert(counter_4 == 7);\n\n    // test while loop expression\n    let result = while true { break; };\n\n    true\n}",
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
