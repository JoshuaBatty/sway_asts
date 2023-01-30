Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb13d3811d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
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
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 68,
                                            end: 70,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 71,
                                            end: 75,
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
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 75,
                                            end: 77,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 78,
                                                    end: 80,
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
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 81,
                                                                end: 85,
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
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 92,
                                                            end: 95,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 96,
                                                                end: 105,
                                                                as_str(): "my_struct",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 106,
                                                            end: 107,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 108,
                                                                        end: 116,
                                                                        as_str(): "MyStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                        fields: Braces {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 127,
                                                                                    end: 128,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 128,
                                                                                            end: 129,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 130,
                                                                                                    end: 131,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 131,
                                                                                end: 132,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 117,
                                                                end: 138,
                                                                as_str(): "{\n        a: 5,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 138,
                                                            end: 139,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 144,
                                                            end: 147,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 148,
                                                                end: 155,
                                                                as_str(): "my_enum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 156,
                                                            end: 157,
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
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 158,
                                                                            end: 164,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 164,
                                                                                end: 166,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 166,
                                                                                    end: 172,
                                                                                    as_str(): "Number",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                    ),
                                                                ],
                                                                incomplete_suffix: false,
                                                            },
                                                        ),
                                                        args: Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 173,
                                                                                    end: 175,
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
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 172,
                                                                end: 176,
                                                                as_str(): "(10)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 176,
                                                            end: 177,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 185,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 205,
                                                                as_str(): "my_struct_with_enum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 207,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 208,
                                                                        end: 224,
                                                                        as_str(): "MyStructWithEnum",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                        fields: Braces {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 235,
                                                                                    end: 236,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 236,
                                                                                            end: 237,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 238,
                                                                                                        end: 247,
                                                                                                        as_str(): "my_struct",
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
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 247,
                                                                                end: 248,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 257,
                                                                                    end: 258,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 258,
                                                                                            end: 259,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 260,
                                                                                                        end: 267,
                                                                                                        as_str(): "my_enum",
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
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 267,
                                                                                end: 268,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 225,
                                                                end: 274,
                                                                as_str(): "{\n        a: my_struct,\n        b: my_enum,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 274,
                                                            end: 275,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 280,
                                                            end: 283,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 284,
                                                                end: 285,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 287,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        String(
                                                            LitString {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 288,
                                                                    end: 295,
                                                                    as_str(): "\"abcde\"",
                                                                },
                                                                parsed: "abcde",
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 296,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 304,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 305,
                                                                end: 306,
                                                                as_str(): "e",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 307,
                                                            end: 308,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 309,
                                                                    end: 313,
                                                                    as_str(): "true",
                                                                },
                                                                kind: True,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 313,
                                                            end: 314,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 319,
                                                            end: 322,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 323,
                                                                end: 324,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 325,
                                                            end: 326,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 327,
                                                                    end: 329,
                                                                    as_str(): "15",
                                                                },
                                                                parsed: 15,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 329,
                                                            end: 330,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 335,
                                                            end: 338,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 339,
                                                                end: 340,
                                                                as_str(): "g",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 341,
                                                            end: 342,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 343,
                                                                    end: 353,
                                                                    as_str(): "0b10101010",
                                                                },
                                                                parsed: 170,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 353,
                                                            end: 354,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 359,
                                                            end: 362,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 363,
                                                                end: 364,
                                                                as_str(): "h",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 364,
                                                                    end: 365,
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
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 366,
                                                                                end: 370,
                                                                                as_str(): "b256",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 371,
                                                            end: 372,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 373,
                                                                    end: 631,
                                                                    as_str(): "0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010",
                                                                },
                                                                parsed: 77194726158210796949047323339125271902179989777093709359638389338608753093290,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 631,
                                                            end: 632,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 638,
                                                                        end: 648,
                                                                        as_str(): "eight_args",
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
                                                            value_separator_pairs: [
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 649,
                                                                                        end: 658,
                                                                                        as_str(): "my_struct",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 658,
                                                                            end: 659,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 660,
                                                                                        end: 667,
                                                                                        as_str(): "my_enum",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 667,
                                                                            end: 668,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 669,
                                                                                        end: 688,
                                                                                        as_str(): "my_struct_with_enum",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 688,
                                                                            end: 689,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 690,
                                                                                        end: 691,
                                                                                        as_str(): "d",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 691,
                                                                            end: 692,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 693,
                                                                                        end: 694,
                                                                                        as_str(): "e",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 694,
                                                                            end: 695,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 696,
                                                                                        end: 697,
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
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 697,
                                                                            end: 698,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 699,
                                                                                        end: 700,
                                                                                        as_str(): "g",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 700,
                                                                            end: 701,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 702,
                                                                                    end: 703,
                                                                                    as_str(): "h",
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
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 648,
                                                            end: 704,
                                                            as_str(): "(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 704,
                                                            end: 705,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 740,
                                                            end: 743,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 744,
                                                                end: 751,
                                                                as_str(): "ls_than",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 752,
                                                            end: 753,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: LessThan {
                                                        lhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 754,
                                                                        end: 755,
                                                                        as_str(): "4",
                                                                    },
                                                                    parsed: 4,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        less_than_token: LessThanToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 756,
                                                                end: 757,
                                                                as_str(): "<",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 759,
                                                                        as_str(): "5",
                                                                    },
                                                                    parsed: 5,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 759,
                                                            end: 760,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 765,
                                                            end: 768,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 769,
                                                                end: 776,
                                                                as_str(): "gt_than",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 777,
                                                            end: 778,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: GreaterThan {
                                                        lhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 779,
                                                                        end: 780,
                                                                        as_str(): "5",
                                                                    },
                                                                    parsed: 5,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        greater_than_token: GreaterThanToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 781,
                                                                end: 782,
                                                                as_str(): ">",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 783,
                                                                        end: 784,
                                                                        as_str(): "4",
                                                                    },
                                                                    parsed: 4,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 784,
                                                            end: 785,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 790,
                                                            end: 793,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 794,
                                                                end: 796,
                                                                as_str(): "le",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 797,
                                                            end: 798,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: LessThanEq {
                                                        lhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 799,
                                                                        end: 800,
                                                                        as_str(): "4",
                                                                    },
                                                                    parsed: 4,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        less_than_eq_token: LessThanEqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 801,
                                                                end: 803,
                                                                as_str(): "<=",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 804,
                                                                        end: 805,
                                                                        as_str(): "4",
                                                                    },
                                                                    parsed: 4,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 805,
                                                            end: 806,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 811,
                                                            end: 814,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 815,
                                                                end: 817,
                                                                as_str(): "ge",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 818,
                                                            end: 819,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: GreaterThanEq {
                                                        lhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 820,
                                                                        end: 821,
                                                                        as_str(): "4",
                                                                    },
                                                                    parsed: 4,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        greater_than_eq_token: GreaterThanEqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 822,
                                                                end: 824,
                                                                as_str(): ">=",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 825,
                                                                        end: 826,
                                                                        as_str(): "4",
                                                                    },
                                                                    parsed: 4,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 826,
                                                            end: 827,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 832,
                                                            end: 835,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 836,
                                                                end: 838,
                                                                as_str(): "eq",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 839,
                                                            end: 840,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Equal {
                                                        lhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 841,
                                                                        end: 842,
                                                                        as_str(): "5",
                                                                    },
                                                                    parsed: 5,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        double_eq_token: DoubleEqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 843,
                                                                end: 845,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 846,
                                                                        end: 847,
                                                                        as_str(): "5",
                                                                    },
                                                                    parsed: 5,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 847,
                                                            end: 848,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 854,
                                                            end: 860,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Literal(
                                                            Bool(
                                                                LitBool {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 861,
                                                                        end: 865,
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
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 865,
                                                            end: 866,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 86,
                                        end: 868,
                                        as_str(): "{\n    let my_struct = MyStruct {\n        a: 5,\n    };\n    let my_enum = MyEnum::Number(10);\n    let my_struct_with_enum = MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    };\n    let d = \"abcde\";\n    let e = true;\n    let f = 15;\n    let g = 0b10101010;\n    let h: b256 = 0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010;\n\n    eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h);\n\n    // test some comparisons\n    let ls_than = 4 < 5;\n    let gt_than = 5 > 4;\n    let le = 4 <= 4;\n    let ge = 4 >= 4;\n    let eq = 5 == 5;\n\n    return true;\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 869,
                                        end: 875,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 876,
                                        end: 884,
                                        as_str(): "MyStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 891,
                                                                end: 892,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 892,
                                                                end: 893,
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
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 894,
                                                                            end: 897,
                                                                            as_str(): "u64",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 897,
                                                        end: 898,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 885,
                                        end: 900,
                                        as_str(): "{\n    a: u64,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 902,
                                        end: 906,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 907,
                                        end: 913,
                                        as_str(): "MyEnum",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 920,
                                                                end: 926,
                                                                as_str(): "Number",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 926,
                                                                end: 927,
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
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 928,
                                                                            end: 931,
                                                                            as_str(): "u64",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 931,
                                                        end: 932,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 937,
                                                                end: 941,
                                                                as_str(): "Unit",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 941,
                                                                end: 942,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 943,
                                                                    end: 945,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 945,
                                                        end: 946,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 914,
                                        end: 948,
                                        as_str(): "{\n    Number: u64,\n    Unit: (),\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 950,
                                        end: 956,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 957,
                                        end: 973,
                                        as_str(): "MyStructWithEnum",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 980,
                                                                end: 981,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 981,
                                                                end: 982,
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
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 983,
                                                                            end: 991,
                                                                            as_str(): "MyStruct",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 991,
                                                        end: 992,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 997,
                                                                end: 998,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 998,
                                                                end: 999,
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
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 1000,
                                                                            end: 1006,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 1006,
                                                        end: 1007,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 974,
                                        end: 1009,
                                        as_str(): "{\n    a: MyStruct,\n    b: MyEnum,\n}",
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
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1011,
                                            end: 1013,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1014,
                                            end: 1024,
                                            as_str(): "eight_args",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1025,
                                                                        end: 1026,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 1026,
                                                                    end: 1027,
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
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 1028,
                                                                                end: 1036,
                                                                                as_str(): "MyStruct",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 1036,
                                                                end: 1037,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1038,
                                                                        end: 1039,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 1039,
                                                                    end: 1040,
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
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 1041,
                                                                                end: 1047,
                                                                                as_str(): "MyEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 1047,
                                                                end: 1048,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1049,
                                                                        end: 1050,
                                                                        as_str(): "c",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 1050,
                                                                    end: 1051,
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
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 1052,
                                                                                end: 1068,
                                                                                as_str(): "MyStructWithEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 1068,
                                                                end: 1069,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1070,
                                                                        end: 1071,
                                                                        as_str(): "d",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 1071,
                                                                    end: 1072,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            ty: Str {
                                                                str_token: StrToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1073,
                                                                        end: 1076,
                                                                        as_str(): "str",
                                                                    },
                                                                },
                                                                length: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 1077,
                                                                                    end: 1078,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1076,
                                                                        end: 1079,
                                                                        as_str(): "[5]",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 1079,
                                                                end: 1080,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1081,
                                                                        end: 1082,
                                                                        as_str(): "e",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 1082,
                                                                    end: 1083,
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
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 1084,
                                                                                end: 1088,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 1088,
                                                                end: 1089,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1090,
                                                                        end: 1091,
                                                                        as_str(): "f",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 1091,
                                                                    end: 1092,
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
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 1093,
                                                                                end: 1096,
                                                                                as_str(): "u64",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 1096,
                                                                end: 1097,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb13d3811d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                        ),
                                                                        start: 1098,
                                                                        end: 1099,
                                                                        as_str(): "g",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 1099,
                                                                    end: 1100,
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
                                                                                src (ptr): 0x00007fb13d3811d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                                ),
                                                                                start: 1101,
                                                                                end: 1103,
                                                                                as_str(): "u8",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 1103,
                                                                end: 1104,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13d3811d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                    ),
                                                                    start: 1105,
                                                                    end: 1106,
                                                                    as_str(): "h",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb13d3811d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                ),
                                                                start: 1106,
                                                                end: 1107,
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
                                                                            src (ptr): 0x00007fb13d3811d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                                            ),
                                                                            start: 1108,
                                                                            end: 1112,
                                                                            as_str(): "b256",
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
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1024,
                                            end: 1113,
                                            as_str(): "(a: MyStruct, b: MyEnum, c: MyStructWithEnum, d: str[5], e: bool, f: u64, g: u8, h: b256)",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 1120,
                                                            end: 1126,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: None,
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 1126,
                                                            end: 1127,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 1114,
                                        end: 1129,
                                        as_str(): "{\n    return;\n}",
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
