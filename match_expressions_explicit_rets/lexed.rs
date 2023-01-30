Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0bd033020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0bd033020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0bd033020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 119,
                                            as_str(): "fn",
                                        },
                                    },
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
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bd033020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                            ),
                                            start: 124,
                                            end: 126,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0bd033020,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                    ),
                                                    start: 127,
                                                    end: 129,
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
                                                                src (ptr): 0x00007fe0bd033020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                ),
                                                                start: 130,
                                                                end: 134,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Match {
                                                match_token: MatchToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bd033020,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                        ),
                                                        start: 141,
                                                        end: 146,
                                                        as_str(): "match",
                                                    },
                                                },
                                                value: Literal(
                                                    Bool(
                                                        LitBool {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bd033020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                ),
                                                                start: 147,
                                                                end: 151,
                                                                as_str(): "true",
                                                            },
                                                            kind: True,
                                                        },
                                                    ),
                                                ),
                                                branches: Braces {
                                                    inner: [
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 162,
                                                                            end: 166,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd033020,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                    ),
                                                                    start: 167,
                                                                    end: 169,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Block {
                                                                block: Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [
                                                                            Expr {
                                                                                expr: Return {
                                                                                    return_token: ReturnToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd033020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                            ),
                                                                                            start: 184,
                                                                                            end: 190,
                                                                                            as_str(): "return",
                                                                                        },
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        Literal(
                                                                                            Bool(
                                                                                                LitBool {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                        ),
                                                                                                        start: 191,
                                                                                                        end: 195,
                                                                                                        as_str(): "true",
                                                                                                    },
                                                                                                    kind: True,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                semicolon_token_opt: Some(
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd033020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                            ),
                                                                                            start: 195,
                                                                                            end: 196,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ],
                                                                        final_expr_opt: None,
                                                                    },
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
                                                                comma_token_opt: Some(
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 206,
                                                                            end: 207,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 216,
                                                                            end: 221,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd033020,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                    ),
                                                                    start: 222,
                                                                    end: 224,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Block {
                                                                block: Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [
                                                                            Expr {
                                                                                expr: Return {
                                                                                    return_token: ReturnToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd033020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                            ),
                                                                                            start: 239,
                                                                                            end: 245,
                                                                                            as_str(): "return",
                                                                                        },
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        Literal(
                                                                                            Bool(
                                                                                                LitBool {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                        ),
                                                                                                        start: 246,
                                                                                                        end: 251,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                    kind: False,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                semicolon_token_opt: Some(
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0bd033020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                            ),
                                                                                            start: 251,
                                                                                            end: 252,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ],
                                                                        final_expr_opt: None,
                                                                    },
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
                                                                comma_token_opt: None,
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bd033020,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                        ),
                                                        start: 152,
                                                        end: 268,
                                                        as_str(): "{\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0bd033020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                        ),
                                        start: 135,
                                        end: 270,
                                        as_str(): "{\n    match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }\n}",
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
