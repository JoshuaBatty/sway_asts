Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a18734b80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a18734b80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
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
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
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
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
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
                                                src (ptr): 0x00007f8a18734b80,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007f8a18734b80,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
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
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
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
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
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
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 41,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                        ),
                                        start: 42,
                                        end: 43,
                                        as_str(): "S",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Annotated {
                                                attribute_list: [],
                                                value: TypeField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 50,
                                                            end: 51,
                                                            as_str(): "t",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 51,
                                                            end: 52,
                                                            as_str(): ":",
                                                        },
                                                    },
                                                    ty: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Path(
                                                                    PathType {
                                                                        root_opt: None,
                                                                        prefix: PathTypeSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                    ),
                                                                                    start: 54,
                                                                                    end: 57,
                                                                                    as_str(): "u64",
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
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 57,
                                                                        end: 58,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 53,
                                                                end: 59,
                                                                as_str(): "(u64,)",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                        ),
                                        start: 44,
                                        end: 61,
                                        as_str(): "{\n    t: (u64,)\n}",
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
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 63,
                                            end: 65,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 70,
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
                                            src (ptr): 0x00007f8a18734b80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                            ),
                                            start: 70,
                                            end: 72,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a18734b80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                    ),
                                                    start: 73,
                                                    end: 75,
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
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 76,
                                                                end: 79,
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
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 86,
                                                            end: 89,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 91,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 92,
                                                            end: 93,
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
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 94,
                                                                        end: 95,
                                                                        as_str(): "S",
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
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    ExprStructField {
                                                                        field_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                ),
                                                                                start: 106,
                                                                                end: 107,
                                                                                as_str(): "t",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                        ),
                                                                                        start: 107,
                                                                                        end: 108,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Tuple(
                                                                                    Parens {
                                                                                        inner: Cons {
                                                                                            head: Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                            ),
                                                                                                            start: 110,
                                                                                                            end: 111,
                                                                                                            as_str(): "2",
                                                                                                        },
                                                                                                        parsed: 2,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                            comma_token: CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                    ),
                                                                                                    start: 111,
                                                                                                    end: 112,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                            tail: Punctuated {
                                                                                                value_separator_pairs: [],
                                                                                                final_value_opt: None,
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                            ),
                                                                                            start: 109,
                                                                                            end: 113,
                                                                                            as_str(): "(2,)",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 96,
                                                                end: 119,
                                                                as_str(): "{\n        t: (2,)\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 120,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 125,
                                                            end: 128,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 130,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 131,
                                                            end: 132,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 133,
                                                                end: 138,
                                                                as_str(): "match",
                                                            },
                                                        },
                                                        value: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 139,
                                                                            end: 140,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                                incomplete_suffix: false,
                                                            },
                                                        ),
                                                        branches: Braces {
                                                            inner: [
                                                                MatchBranch {
                                                                    pattern: Struct {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a18734b80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                        ),
                                                                                        start: 151,
                                                                                        end: 152,
                                                                                        as_str(): "S",
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
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
                                                                                    Field {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                                ),
                                                                                                start: 155,
                                                                                                end: 156,
                                                                                                as_str(): "t",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: None,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                ),
                                                                                start: 153,
                                                                                end: 158,
                                                                                as_str(): "{ t }",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 159,
                                                                            end: 161,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Expr {
                                                                        expr: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                            ),
                                                                                            start: 162,
                                                                                            end: 163,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                ),
                                                                                start: 163,
                                                                                end: 164,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007f8a18734b80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                ),
                                                                start: 141,
                                                                end: 170,
                                                                as_str(): "{\n        S { t } => t,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 171,
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
                                                                        src (ptr): 0x00007f8a18734b80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                        ),
                                                                        start: 176,
                                                                        end: 182,
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
                                                                    lhs: TupleFieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a18734b80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                            ),
                                                                                            start: 183,
                                                                                            end: 184,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a18734b80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                ),
                                                                                start: 184,
                                                                                end: 185,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 185,
                                                                            end: 186,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a18734b80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                            ),
                                                                            start: 187,
                                                                            end: 189,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a18734b80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                                                    ),
                                                                                    start: 190,
                                                                                    end: 191,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 192,
                                                            as_str(): "(b.0 == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 192,
                                                            end: 193,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a18734b80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 200,
                                                            as_str(): "1",
                                                        },
                                                        parsed: 1,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a18734b80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReaLAGJ/tuple_single_element/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 202,
                                        as_str(): "{\n    let a = S {\n        t: (2,)\n    };\n    let b = match a {\n        S { t } => t,\n    };\n    assert(b.0 == 2);\n\n    1\n}",
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
