Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb13ad4b060,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb13ad4b060,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
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
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 17,
                                            as_str(): "abort",
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
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 17,
                                            end: 19,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb13ad4b060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                    ),
                                                    start: 20,
                                                    end: 22,
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
                                                                src (ptr): 0x00007fb13ad4b060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                ),
                                                                start: 23,
                                                                end: 27,
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
                                            Asm(
                                                AsmBlock {
                                                    asm_token: AsmToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13ad4b060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                            ),
                                                            start: 34,
                                                            end: 37,
                                                            as_str(): "asm",
                                                        },
                                                    },
                                                    registers: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb13ad4b060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                            ),
                                                            start: 37,
                                                            end: 39,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    contents: Braces {
                                                        inner: AsmBlockContents {
                                                            instructions: [],
                                                            final_expr_opt: Some(
                                                                AsmFinalExpr {
                                                                    register: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13ad4b060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                            ),
                                                                            start: 50,
                                                                            end: 53,
                                                                            as_str(): "one",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    ty_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                    ),
                                                                                    start: 53,
                                                                                    end: 54,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Path(
                                                                                PathType {
                                                                                    root_opt: None,
                                                                                    prefix: PathTypeSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                ),
                                                                                                start: 55,
                                                                                                end: 59,
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
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb13ad4b060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                            ),
                                                            start: 40,
                                                            end: 77,
                                                            as_str(): "{\n        one: bool // Failure.\n    }",
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13ad4b060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                        ),
                                        start: 28,
                                        end: 79,
                                        as_str(): "{\n    asm() {\n        one: bool // Failure.\n    }\n}",
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
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 83,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 84,
                                            end: 88,
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
                                            src (ptr): 0x00007fb13ad4b060,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                            ),
                                            start: 88,
                                            end: 90,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb13ad4b060,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                    ),
                                                    start: 91,
                                                    end: 93,
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
                                                                src (ptr): 0x00007fb13ad4b060,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 97,
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
                                                            src (ptr): 0x00007fb13ad4b060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 106,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        LogicalAnd {
                                                            lhs: Parens(
                                                                Parens {
                                                                    inner: LogicalAnd {
                                                                        lhs: Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                        ),
                                                                                        start: 108,
                                                                                        end: 112,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        double_ampersand_token: DoubleAmpersandToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                ),
                                                                                start: 113,
                                                                                end: 115,
                                                                                as_str(): "&&",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                        ),
                                                                                        start: 116,
                                                                                        end: 121,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                    kind: False,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                        ),
                                                                        start: 107,
                                                                        end: 122,
                                                                        as_str(): "(true && false)",
                                                                    },
                                                                },
                                                            ),
                                                            double_ampersand_token: DoubleAmpersandToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                    ),
                                                                    start: 123,
                                                                    end: 125,
                                                                    as_str(): "&&",
                                                                },
                                                            },
                                                            rhs: FuncApp {
                                                                func: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                    ),
                                                                                    start: 126,
                                                                                    end: 131,
                                                                                    as_str(): "abort",
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
                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                        ),
                                                                        start: 131,
                                                                        end: 133,
                                                                        as_str(): "()",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                ),
                                                                                start: 164,
                                                                                end: 165,
                                                                                as_str(): "2",
                                                                            },
                                                                            parsed: 2,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb13ad4b060,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 171,
                                                            as_str(): "{\n        // Failure.\n        2\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                    ),
                                                                    start: 172,
                                                                    end: 176,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Continue(
                                                                IfExpr {
                                                                    if_token: IfToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13ad4b060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                            ),
                                                                            start: 177,
                                                                            end: 179,
                                                                            as_str(): "if",
                                                                        },
                                                                    },
                                                                    condition: Expr(
                                                                        LogicalOr {
                                                                            lhs: Parens(
                                                                                Parens {
                                                                                    inner: LogicalOr {
                                                                                        lhs: Literal(
                                                                                            Bool(
                                                                                                LitBool {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                        ),
                                                                                                        start: 181,
                                                                                                        end: 186,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                    kind: False,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        double_pipe_token: DoublePipeToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                ),
                                                                                                start: 187,
                                                                                                end: 189,
                                                                                                as_str(): "||",
                                                                                            },
                                                                                        },
                                                                                        rhs: Literal(
                                                                                            Bool(
                                                                                                LitBool {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                        ),
                                                                                                        start: 190,
                                                                                                        end: 194,
                                                                                                        as_str(): "true",
                                                                                                    },
                                                                                                    kind: True,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                        ),
                                                                                        start: 180,
                                                                                        end: 195,
                                                                                        as_str(): "(false || true)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            double_pipe_token: DoublePipeToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                    ),
                                                                                    start: 196,
                                                                                    end: 198,
                                                                                    as_str(): "||",
                                                                                },
                                                                            },
                                                                            rhs: FuncApp {
                                                                                func: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                    ),
                                                                                                    start: 199,
                                                                                                    end: 204,
                                                                                                    as_str(): "abort",
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
                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                        ),
                                                                                        start: 204,
                                                                                        end: 206,
                                                                                        as_str(): "()",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    then_block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: Some(
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb13ad4b060,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                ),
                                                                                                start: 237,
                                                                                                end: 239,
                                                                                                as_str(): "42",
                                                                                            },
                                                                                            parsed: 42,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13ad4b060,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                            ),
                                                                            start: 207,
                                                                            end: 245,
                                                                            as_str(): "{\n        // Success.\n        42\n    }",
                                                                        },
                                                                    },
                                                                    else_opt: Some(
                                                                        (
                                                                            ElseToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13ad4b060,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                    ),
                                                                                    start: 246,
                                                                                    end: 250,
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
                                                                                                            src (ptr): 0x00007fb13ad4b060,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                                            ),
                                                                                                            start: 281,
                                                                                                            end: 282,
                                                                                                            as_str(): "3",
                                                                                                        },
                                                                                                        parsed: 3,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13ad4b060,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                                                                        ),
                                                                                        start: 251,
                                                                                        end: 288,
                                                                                        as_str(): "{\n        // Failure.\n        3\n    }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13ad4b060,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGj4Pvd/bool_and_or/src/main.sw",
                                        ),
                                        start: 98,
                                        end: 290,
                                        as_str(): "{\n    if (true && false) && abort() {\n        // Failure.\n        2\n    } else if (false || true) || abort() {\n        // Success.\n        42\n    } else {\n        // Failure.\n        3\n    }\n}",
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
