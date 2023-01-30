Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0ceb55870,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0ceb55870,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                                                src (ptr): 0x00007fe0ceb55870,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0ceb55870,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                                                    src (ptr): 0x00007fe0ceb55870,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
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
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 38,
                                            end: 39,
                                            as_str(): "f",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ceb55870,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                    ),
                                                                    start: 40,
                                                                    end: 44,
                                                                    as_str(): "cond",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ceb55870,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                ),
                                                                start: 44,
                                                                end: 45,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                            ),
                                                                            start: 46,
                                                                            end: 50,
                                                                            as_str(): "bool",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 39,
                                            end: 51,
                                            as_str(): "(cond: bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0ceb55870,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                    ),
                                                    start: 52,
                                                    end: 54,
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
                                                                src (ptr): 0x00007fe0ceb55870,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                ),
                                                                start: 55,
                                                                end: 58,
                                                                as_str(): "u64",
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 65,
                                                            end: 67,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                            ),
                                                                            start: 68,
                                                                            end: 72,
                                                                            as_str(): "cond",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                                incomplete_suffix: false,
                                                            },
                                                        ),
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ceb55870,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                ),
                                                                                start: 83,
                                                                                end: 85,
                                                                                as_str(): "10",
                                                                            },
                                                                            parsed: 10,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 73,
                                                            end: 91,
                                                            as_str(): "{\n        10\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ceb55870,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                    ),
                                                                    start: 92,
                                                                    end: 96,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                            ),
                                                                                            start: 107,
                                                                                            end: 109,
                                                                                            as_str(): "20",
                                                                                        },
                                                                                        parsed: 20,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                        ),
                                                                        start: 97,
                                                                        end: 115,
                                                                        as_str(): "{\n        20\n    }",
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
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 117,
                                        as_str(): "{\n    if cond {\n        10\n    } else {\n        20\n    }\n}",
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
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 119,
                                            end: 121,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 122,
                                            end: 126,
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
                                            src (ptr): 0x00007fe0ceb55870,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                            ),
                                            start: 126,
                                            end: 128,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
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
                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                        ),
                                                                        start: 135,
                                                                        end: 136,
                                                                        as_str(): "f",
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
                                                                Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ceb55870,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                ),
                                                                                start: 137,
                                                                                end: 141,
                                                                                as_str(): "true",
                                                                            },
                                                                            kind: True,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 136,
                                                            end: 142,
                                                            as_str(): "(true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 142,
                                                            end: 143,
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
                                                                        src (ptr): 0x00007fe0ceb55870,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                        ),
                                                                        start: 148,
                                                                        end: 154,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                            ),
                                                                                            start: 155,
                                                                                            end: 156,
                                                                                            as_str(): "f",
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ceb55870,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                                    ),
                                                                                                    start: 157,
                                                                                                    end: 162,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ceb55870,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                ),
                                                                                start: 156,
                                                                                end: 163,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ceb55870,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                            ),
                                                                            start: 164,
                                                                            end: 166,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ceb55870,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                                                    ),
                                                                                    start: 167,
                                                                                    end: 169,
                                                                                    as_str(): "20",
                                                                                },
                                                                                parsed: 20,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 154,
                                                            end: 170,
                                                            as_str(): "(f(false) == 20)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ceb55870,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 171,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0ceb55870,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbXAh5V/inline_if_expr_const/src/main.sw",
                                        ),
                                        start: 129,
                                        end: 173,
                                        as_str(): "{\n    f(true);\n    assert(f(false) == 20);\n}",
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
