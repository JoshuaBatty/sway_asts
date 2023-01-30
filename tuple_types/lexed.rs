Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a28725630,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a28725630,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 24,
                                            as_str(): "gimme_a_unit",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 24,
                                            end: 26,
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
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 33,
                                                            end: 36,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 37,
                                                                end: 38,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a28725630,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                    ),
                                                                    start: 38,
                                                                    end: 39,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Tuple(
                                                                Parens {
                                                                    inner: Nil,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 40,
                                                                        end: 42,
                                                                        as_str(): "()",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 43,
                                                            end: 44,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Nil,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 45,
                                                                end: 47,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 47,
                                                            end: 48,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 53,
                                                                end: 54,
                                                                as_str(): "x",
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
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 56,
                                        as_str(): "{\n    let x: () = ();\n    x\n}",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 58,
                                            end: 60,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 61,
                                            end: 78,
                                            as_str(): "also_gimme_a_unit",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 80,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 81,
                                                    end: 83,
                                                    as_str(): "->",
                                                },
                                            },
                                            Tuple(
                                                Parens {
                                                    inner: Nil,
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28725630,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                        ),
                                                        start: 84,
                                                        end: 86,
                                                        as_str(): "()",
                                                    },
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
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 93,
                                                            end: 96,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 97,
                                                                end: 98,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a28725630,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                    ),
                                                                    start: 98,
                                                                    end: 99,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Tuple(
                                                                Parens {
                                                                    inner: Nil,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 100,
                                                                        end: 102,
                                                                        as_str(): "()",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 103,
                                                            end: 104,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Nil,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 107,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 107,
                                                            end: 108,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 113,
                                                                end: 114,
                                                                as_str(): "x",
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
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 87,
                                        end: 116,
                                        as_str(): "{\n    let x: () = ();\n    x\n}",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 118,
                                            end: 120,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 121,
                                            end: 141,
                                            as_str(): "gimme_a_single_value",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 141,
                                            end: 143,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 144,
                                                    end: 146,
                                                    as_str(): "->",
                                                },
                                            },
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a28725630,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                            ),
                                                                            start: 148,
                                                                            end: 151,
                                                                            as_str(): "u32",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 151,
                                                                end: 152,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28725630,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                        ),
                                                        start: 147,
                                                        end: 153,
                                                        as_str(): "(u32,)",
                                                    },
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
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 160,
                                                            end: 163,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 164,
                                                                end: 165,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a28725630,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                    ),
                                                                    start: 165,
                                                                    end: 166,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Tuple(
                                                                Parens {
                                                                    inner: Cons {
                                                                        head: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a28725630,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                                            ),
                                                                                            start: 168,
                                                                                            end: 171,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                            },
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a28725630,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                                ),
                                                                                start: 171,
                                                                                end: 172,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                        tail: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: None,
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 167,
                                                                        end: 173,
                                                                        as_str(): "(u32,)",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 174,
                                                            end: 175,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a28725630,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                                ),
                                                                                start: 177,
                                                                                end: 180,
                                                                                as_str(): "123",
                                                                            },
                                                                            parsed: 123,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U32,
                                                                                    Span {
                                                                                        src (ptr): 0x00007f8a28725630,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                                        ),
                                                                                        start: 180,
                                                                                        end: 183,
                                                                                        as_str(): "u32",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 183,
                                                                        end: 184,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 176,
                                                                end: 185,
                                                                as_str(): "(123u32,)",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 185,
                                                            end: 186,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 191,
                                                                end: 192,
                                                                as_str(): "x",
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
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 154,
                                        end: 194,
                                        as_str(): "{\n    let x: (u32,) = (123u32,);\n    x\n}",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 196,
                                            end: 198,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 199,
                                            end: 211,
                                            as_str(): "gimme_a_pair",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 211,
                                            end: 213,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 214,
                                                    end: 216,
                                                    as_str(): "->",
                                                },
                                            },
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a28725630,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                            ),
                                                                            start: 218,
                                                                            end: 221,
                                                                            as_str(): "u32",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 221,
                                                                end: 222,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathType {
                                                                        root_opt: None,
                                                                        prefix: PathTypeSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a28725630,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                                    ),
                                                                                    start: 223,
                                                                                    end: 226,
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
                                                        },
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28725630,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                        ),
                                                        start: 217,
                                                        end: 227,
                                                        as_str(): "(u32, u64)",
                                                    },
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
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 235,
                                                                        end: 236,
                                                                        as_str(): "1",
                                                                    },
                                                                    parsed: 1,
                                                                    ty_opt: Some(
                                                                        (
                                                                            U32,
                                                                            Span {
                                                                                src (ptr): 0x00007f8a28725630,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                                ),
                                                                                start: 236,
                                                                                end: 239,
                                                                                as_str(): "u32",
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 239,
                                                                end: 240,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a28725630,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                                ),
                                                                                start: 241,
                                                                                end: 242,
                                                                                as_str(): "2",
                                                                            },
                                                                            parsed: 2,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007f8a28725630,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                                        ),
                                                                                        start: 242,
                                                                                        end: 245,
                                                                                        as_str(): "u64",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28725630,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                        ),
                                                        start: 234,
                                                        end: 246,
                                                        as_str(): "(1u32, 2u64)",
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 228,
                                        end: 248,
                                        as_str(): "{\n    (1u32, 2u64)\n}",
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 250,
                                            end: 252,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 253,
                                            end: 257,
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
                                            src (ptr): 0x00007f8a28725630,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                            ),
                                            start: 257,
                                            end: 259,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a28725630,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                    ),
                                                    start: 260,
                                                    end: 262,
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
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 263,
                                                                end: 266,
                                                                as_str(): "u32",
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
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 273,
                                                                        end: 285,
                                                                        as_str(): "gimme_a_unit",
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
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 285,
                                                            end: 287,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 287,
                                                            end: 288,
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
                                                                        src (ptr): 0x00007f8a28725630,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                        ),
                                                                        start: 293,
                                                                        end: 310,
                                                                        as_str(): "also_gimme_a_unit",
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
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 310,
                                                            end: 312,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 312,
                                                            end: 313,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 318,
                                                            end: 321,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 322,
                                                                end: 323,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 324,
                                                            end: 325,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: FuncApp {
                                                        func: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a28725630,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                            ),
                                                                            start: 326,
                                                                            end: 346,
                                                                            as_str(): "gimme_a_single_value",
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
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 346,
                                                                end: 348,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 348,
                                                            end: 349,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 357,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 358,
                                                                end: 359,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 360,
                                                            end: 361,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: FuncApp {
                                                        func: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a28725630,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                            ),
                                                                            start: 362,
                                                                            end: 374,
                                                                            as_str(): "gimme_a_pair",
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
                                                                src (ptr): 0x00007f8a28725630,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                                ),
                                                                start: 374,
                                                                end: 376,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 376,
                                                            end: 377,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a28725630,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                                            ),
                                                            start: 382,
                                                            end: 385,
                                                            as_str(): "123",
                                                        },
                                                        parsed: 123,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a28725630,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuJWxkG/tuple_types/src/main.sw",
                                        ),
                                        start: 267,
                                        end: 387,
                                        as_str(): "{\n    gimme_a_unit();\n    also_gimme_a_unit();\n    let x = gimme_a_single_value();\n    let b = gimme_a_pair();\n    123\n}",
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
