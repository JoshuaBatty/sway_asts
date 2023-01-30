Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe07c8bc640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
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
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
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
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 17,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 17,
                                            end: 19,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Glob {
                                        star_token: StarToken {
                                            span: Span {
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 19,
                                                end: 20,
                                                as_str(): "*",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 20,
                                        end: 21,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 22,
                                        end: 25,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 26,
                                            end: 29,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 29,
                                            end: 31,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 31,
                                                end: 37,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe07c8bc640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                ),
                                                start: 37,
                                                end: 39,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 39,
                                                    end: 45,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 46,
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
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 50,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 51,
                                            end: 54,
                                            as_str(): "foo",
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
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 55,
                                                                        end: 59,
                                                                        as_str(): "init",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 59,
                                                                    end: 60,
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
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 61,
                                                                                end: 64,
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
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 65,
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
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 67,
                                                                    as_str(): "n",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 67,
                                                                end: 68,
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
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 69,
                                                                            end: 72,
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 54,
                                            end: 73,
                                            as_str(): "(init: u64, n: u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 74,
                                                    end: 76,
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
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 77,
                                                                end: 80,
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
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 87,
                                                            end: 90,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 91,
                                                                    end: 94,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 95,
                                                                end: 100,
                                                                as_str(): "index",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 102,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 103,
                                                                    end: 104,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 105,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 110,
                                                            end: 113,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 114,
                                                                    end: 117,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 121,
                                                                as_str(): "sum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 122,
                                                            end: 123,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 124,
                                                                    end: 125,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 125,
                                                            end: 126,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 131,
                                                            end: 136,
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
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 137,
                                                                            end: 142,
                                                                            as_str(): "index",
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
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 143,
                                                                end: 144,
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
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 145,
                                                                            end: 146,
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
                                                    block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: If(
                                                                        IfExpr {
                                                                            if_token: IfToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 157,
                                                                                    end: 159,
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
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 160,
                                                                                                            end: 165,
                                                                                                            as_str(): "index",
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
                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                ),
                                                                                                start: 166,
                                                                                                end: 167,
                                                                                                as_str(): "%",
                                                                                            },
                                                                                        },
                                                                                        rhs: Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                        ),
                                                                                                        start: 168,
                                                                                                        end: 169,
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
                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                            ),
                                                                                            start: 170,
                                                                                            end: 172,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                    },
                                                                                    rhs: Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                    ),
                                                                                                    start: 173,
                                                                                                    end: 174,
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
                                                                                            expr: Reassignment {
                                                                                                assignable: Var(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 189,
                                                                                                            end: 192,
                                                                                                            as_str(): "sum",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                reassignment_op: ReassignmentOp {
                                                                                                    variant: Equals,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                        ),
                                                                                                        start: 193,
                                                                                                        end: 194,
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
                                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 195,
                                                                                                                        end: 198,
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
                                                                                                    add_token: AddToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 199,
                                                                                                            end: 200,
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
                                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 201,
                                                                                                                        end: 206,
                                                                                                                        as_str(): "index",
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
                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                        ),
                                                                                                        start: 206,
                                                                                                        end: 207,
                                                                                                        as_str(): ";",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ],
                                                                                    final_expr_opt: None,
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 175,
                                                                                    end: 217,
                                                                                    as_str(): "{\n            sum = sum + index;\n        }",
                                                                                },
                                                                            },
                                                                            else_opt: None,
                                                                        },
                                                                    ),
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 217,
                                                                                end: 218,
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
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 227,
                                                                                    end: 232,
                                                                                    as_str(): "index",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: Equals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 233,
                                                                                end: 234,
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
                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                ),
                                                                                                start: 235,
                                                                                                end: 240,
                                                                                                as_str(): "index",
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
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 241,
                                                                                    end: 242,
                                                                                    as_str(): "+",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                            ),
                                                                                            start: 243,
                                                                                            end: 244,
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
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 244,
                                                                                end: 245,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 147,
                                                            end: 251,
                                                            as_str(): "{\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Add {
                                                lhs: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 256,
                                                                    end: 259,
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
                                                add_token: AddToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 260,
                                                        end: 261,
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
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 262,
                                                                    end: 266,
                                                                    as_str(): "init",
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
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 81,
                                        end: 269,
                                        as_str(): "{\n    let mut index = 0;\n    let mut sum = 0;\n    while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }\n    sum + init \n}",
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
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 271,
                                            end: 273,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 274,
                                            end: 278,
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
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 278,
                                            end: 280,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 281,
                                                    end: 283,
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
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 284,
                                                                end: 288,
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
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 298,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 299,
                                                                end: 300,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 302,
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
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 303,
                                                                            end: 306,
                                                                            as_str(): "foo",
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
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                        ),
                                                                                        start: 307,
                                                                                        end: 309,
                                                                                        as_str(): "11",
                                                                                    },
                                                                                    parsed: 11,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 309,
                                                                                end: 310,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 311,
                                                                                    end: 312,
                                                                                    as_str(): "4",
                                                                                },
                                                                                parsed: 4,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 306,
                                                                end: 313,
                                                                as_str(): "(11, 4)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 313,
                                                            end: 314,
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
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 319,
                                                                        end: 325,
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
                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                        ),
                                                                                        start: 326,
                                                                                        end: 327,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 328,
                                                                            end: 330,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 331,
                                                                                    end: 333,
                                                                                    as_str(): "13",
                                                                                },
                                                                                parsed: 13,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 325,
                                                            end: 334,
                                                            as_str(): "(x == 13)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 334,
                                                            end: 335,
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
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 344,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe07c8bc640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                        ),
                                        start: 289,
                                        end: 346,
                                        as_str(): "{\n    let x = foo(11, 4);\n    assert(x == 13);\n    true\n}",
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
