Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe07c9e90e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe07c9e90e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe07c9e90e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 14,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe07c9e90e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                        ),
                                        start: 15,
                                        end: 25,
                                        as_str(): "GLOBAL_NUM",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe07c9e90e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 27,
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
                                                        src (ptr): 0x00007fe07c9e90e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                        ),
                                                        start: 28,
                                                        end: 36,
                                                        as_str(): "a_number",
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
                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                    ),
                                                                    start: 37,
                                                                    end: 38,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    CommaToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c9e90e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                            ),
                                                            start: 38,
                                                            end: 39,
                                                            as_str(): ",",
                                                        },
                                                    },
                                                ),
                                                (
                                                    Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                    ),
                                                                    start: 40,
                                                                    end: 41,
                                                                    as_str(): "2",
                                                                },
                                                                parsed: 2,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    CommaToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c9e90e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                            ),
                                                            start: 41,
                                                            end: 42,
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
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 43,
                                                                end: 44,
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 36,
                                            end: 45,
                                            as_str(): "(1, 2, 3)",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe07c9e90e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 50,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 51,
                                            end: 59,
                                            as_str(): "a_number",
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
                                                                        src (ptr): 0x00007fe07c9e90e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                        ),
                                                                        start: 60,
                                                                        end: 62,
                                                                        as_str(): "_a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                    ),
                                                                    start: 62,
                                                                    end: 63,
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
                                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                                ),
                                                                                start: 64,
                                                                                end: 67,
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
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 67,
                                                                end: 68,
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
                                                                        src (ptr): 0x00007fe07c9e90e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                        ),
                                                                        start: 69,
                                                                        end: 71,
                                                                        as_str(): "_b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 72,
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
                                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                                ),
                                                                                start: 73,
                                                                                end: 76,
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
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 76,
                                                                end: 77,
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
                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                    ),
                                                                    start: 78,
                                                                    end: 80,
                                                                    as_str(): "_c",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 80,
                                                                end: 81,
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
                                                                            src (ptr): 0x00007fe07c9e90e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                            ),
                                                                            start: 82,
                                                                            end: 85,
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 86,
                                            as_str(): "(_a: u64, _b: u64, _c: u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 87,
                                                    end: 89,
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
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 93,
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
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c9e90e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 102,
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
                                        src (ptr): 0x00007fe07c9e90e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                        ),
                                        start: 94,
                                        end: 104,
                                        as_str(): "{\n    42\n}",
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 106,
                                            end: 108,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 109,
                                            end: 113,
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
                                            src (ptr): 0x00007fe07c9e90e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                            ),
                                            start: 113,
                                            end: 115,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe07c9e90e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                    ),
                                                    start: 116,
                                                    end: 118,
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
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 119,
                                                                end: 122,
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
                                                            src (ptr): 0x00007fe07c9e90e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                            ),
                                                            start: 129,
                                                            end: 132,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 133,
                                                                end: 134,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c9e90e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 136,
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
                                                                            src (ptr): 0x00007fe07c9e90e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                            ),
                                                                            start: 137,
                                                                            end: 145,
                                                                            as_str(): "a_number",
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
                                                                                        src (ptr): 0x00007fe07c9e90e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 146,
                                                                                        end: 147,
                                                                                        as_str(): "4",
                                                                                    },
                                                                                    parsed: 4,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                                ),
                                                                                start: 147,
                                                                                end: 148,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c9e90e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 149,
                                                                                        end: 150,
                                                                                        as_str(): "5",
                                                                                    },
                                                                                    parsed: 5,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                                ),
                                                                                start: 150,
                                                                                end: 151,
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
                                                                                    src (ptr): 0x00007fe07c9e90e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 152,
                                                                                    end: 153,
                                                                                    as_str(): "6",
                                                                                },
                                                                                parsed: 6,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 145,
                                                                end: 154,
                                                                as_str(): "(4, 5, 6)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c9e90e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                            ),
                                                            start: 154,
                                                            end: 155,
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
                                                                src (ptr): 0x00007fe07c9e90e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                                                ),
                                                                start: 160,
                                                                end: 170,
                                                                as_str(): "GLOBAL_NUM",
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
                                        src (ptr): 0x00007fe07c9e90e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReBF19R/non_literal_const_decl/src/main.sw",
                                        ),
                                        start: 123,
                                        end: 172,
                                        as_str(): "{\n    let a = a_number(4, 5, 6);\n    GLOBAL_NUM\n}",
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
