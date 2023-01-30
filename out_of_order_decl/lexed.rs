Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
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
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 90,
                                            end: 92,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 93,
                                            end: 97,
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
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 97,
                                            end: 99,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 100,
                                                    end: 102,
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 103,
                                                                end: 107,
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
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 117,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 119,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 120,
                                                            end: 121,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 122,
                                                                    end: 124,
                                                                    as_str(): "42",
                                                                },
                                                                parsed: 42,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 124,
                                                            end: 125,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 153,
                                                            end: 156,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 158,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 159,
                                                            end: 160,
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 161,
                                                                            end: 176,
                                                                            as_str(): "the_number_five",
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 176,
                                                                end: 178,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 178,
                                                            end: 179,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 209,
                                                            end: 212,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 213,
                                                                end: 214,
                                                                as_str(): "z",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 215,
                                                            end: 216,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 217,
                                                                        end: 223,
                                                                        as_str(): "AnEnum",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 223,
                                                                            end: 225,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 225,
                                                                                end: 232,
                                                                                as_str(): "Variant",
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
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 232,
                                                            end: 233,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 268,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 269,
                                                                end: 270,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 271,
                                                            end: 272,
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
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 273,
                                                                        end: 283,
                                                                        as_str(): "FuelStruct",
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
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 294,
                                                                                    end: 295,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 295,
                                                                                            end: 296,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 297,
                                                                                                    end: 301,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 301,
                                                                                end: 302,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 311,
                                                                                    end: 312,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 312,
                                                                                            end: 313,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 314,
                                                                                                    end: 319,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 319,
                                                                                end: 320,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 284,
                                                                end: 326,
                                                                as_str(): "{\n        a: true,\n        b: false,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 326,
                                                            end: 327,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 391,
                                                            end: 394,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 395,
                                                                end: 396,
                                                                as_str(): "u",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 397,
                                                            end: 398,
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
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 399,
                                                                        end: 410,
                                                                        as_str(): "FuelWrapper",
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
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 421,
                                                                                    end: 422,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 422,
                                                                                            end: 423,
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
                                                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 424,
                                                                                                        end: 425,
                                                                                                        as_str(): "y",
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
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 425,
                                                                                end: 426,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 435,
                                                                                    end: 436,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 436,
                                                                                            end: 437,
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
                                                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 438,
                                                                                                        end: 439,
                                                                                                        as_str(): "z",
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
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 439,
                                                                                end: 440,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 411,
                                                                end: 446,
                                                                as_str(): "{\n        a: y,\n        b: z,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 446,
                                                            end: 447,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 453,
                                                            end: 456,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 457,
                                                                end: 458,
                                                                as_str(): "v",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 459,
                                                            end: 460,
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 461,
                                                                            end: 472,
                                                                            as_str(): "WrapperEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 472,
                                                                                end: 474,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 474,
                                                                                    end: 481,
                                                                                    as_str(): "Variant",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 482,
                                                                                        end: 483,
                                                                                        as_str(): "u",
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 481,
                                                                end: 484,
                                                                as_str(): "(u)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 484,
                                                            end: 485,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: If(
                                                    IfExpr {
                                                        if_token: IfToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 491,
                                                                end: 493,
                                                                as_str(): "if",
                                                            },
                                                        },
                                                        condition: Let {
                                                            let_token: LetToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 494,
                                                                    end: 497,
                                                                    as_str(): "let",
                                                                },
                                                            },
                                                            lhs: Constant(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 498,
                                                                                end: 504,
                                                                                as_str(): "AnEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 504,
                                                                                    end: 506,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 506,
                                                                                        end: 513,
                                                                                        as_str(): "Variant",
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
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 514,
                                                                    end: 515,
                                                                    as_str(): "=",
                                                                },
                                                            },
                                                            rhs: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 516,
                                                                                end: 522,
                                                                                as_str(): "AnEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 522,
                                                                                    end: 524,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 524,
                                                                                        end: 531,
                                                                                        as_str(): "Variant",
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
                                                        },
                                                        then_block: Braces {
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
                                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 542,
                                                                                                end: 546,
                                                                                                as_str(): "void",
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
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 546,
                                                                                    end: 548,
                                                                                    as_str(): "()",
                                                                                },
                                                                            },
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 548,
                                                                                    end: 549,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 532,
                                                                end: 555,
                                                                as_str(): "{\n        void();\n    }",
                                                            },
                                                        },
                                                        else_opt: None,
                                                    },
                                                ),
                                                semicolon_token_opt: None,
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 561,
                                                            end: 565,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 108,
                                        end: 567,
                                        as_str(): "{\n    let a = 42;\n\n    // fn before decl\n    let x = the_number_five();\n\n    // enum before decl\n    let z = AnEnum::Variant;\n\n    // struct before decl\n    let y = FuelStruct {\n        a: true,\n        b: false,\n    };\n\n    // struct and enum with complex members, out of order\n    let u = FuelWrapper {\n        a: y,\n        b: z,\n    };\n\n    let v = WrapperEnum::Variant(u);\n\n    if let AnEnum::Variant = AnEnum::Variant {\n        void();\n    }\n\n    true\n}",
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
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 569,
                                        end: 575,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 576,
                                        end: 587,
                                        as_str(): "FuelWrapper",
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 594,
                                                                end: 595,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 595,
                                                                end: 596,
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 597,
                                                                            end: 607,
                                                                            as_str(): "FuelStruct",
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
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 607,
                                                        end: 608,
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 613,
                                                                end: 614,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 614,
                                                                end: 615,
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 616,
                                                                            end: 622,
                                                                            as_str(): "AnEnum",
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
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 622,
                                                        end: 623,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 588,
                                        end: 625,
                                        as_str(): "{\n    a: FuelStruct,\n    b: AnEnum,\n}",
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
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 627,
                                        end: 631,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 632,
                                        end: 643,
                                        as_str(): "WrapperEnum",
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 650,
                                                                end: 657,
                                                                as_str(): "Variant",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 657,
                                                                end: 658,
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 659,
                                                                            end: 670,
                                                                            as_str(): "FuelWrapper",
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
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 670,
                                                        end: 671,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 644,
                                        end: 673,
                                        as_str(): "{\n    Variant: FuelWrapper,\n}",
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
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 675,
                                        end: 681,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 682,
                                        end: 692,
                                        as_str(): "FuelStruct",
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 699,
                                                                end: 700,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 700,
                                                                end: 701,
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 702,
                                                                            end: 706,
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
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 706,
                                                        end: 707,
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 712,
                                                                end: 713,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 713,
                                                                end: 714,
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 715,
                                                                            end: 719,
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
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 719,
                                                        end: 720,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 693,
                                        end: 722,
                                        as_str(): "{\n    a: bool,\n    b: bool,\n}",
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
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 724,
                                            end: 726,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 727,
                                            end: 742,
                                            as_str(): "the_number_five",
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
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 742,
                                            end: 744,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 745,
                                                    end: 747,
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 748,
                                                                end: 751,
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
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 752,
                                        end: 761,
                                        as_str(): "{\n    5\n}",
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
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 763,
                                        end: 767,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 768,
                                        end: 774,
                                        as_str(): "AnEnum",
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
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 781,
                                                                end: 788,
                                                                as_str(): "Variant",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 788,
                                                                end: 789,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 790,
                                                                    end: 792,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 792,
                                                        end: 793,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 775,
                                        end: 795,
                                        as_str(): "{\n    Variant: (),\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 818,
                                        end: 822,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: Some(
                                    (
                                        PathType {
                                            root_opt: None,
                                            prefix: PathTypeSegment {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 823,
                                                        end: 832,
                                                        as_str(): "FuelTrait",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe06cba43c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                ),
                                                start: 833,
                                                end: 836,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 837,
                                                    end: 840,
                                                    as_str(): "u64",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                    },
                                ),
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 847,
                                                            end: 849,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 850,
                                                            end: 853,
                                                            as_str(): "foo",
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
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 853,
                                                            end: 855,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 856,
                                                                    end: 858,
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
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 859,
                                                                                end: 863,
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
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 874,
                                                                            end: 878,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 864,
                                                        end: 884,
                                                        as_str(): "{\n        true\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 841,
                                        end: 886,
                                        as_str(): "{\n    fn foo() -> bool {\n        true\n    }\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: None,
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 888,
                                        end: 893,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 894,
                                        end: 903,
                                        as_str(): "FuelTrait",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: None,
                                trait_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 910,
                                                            end: 912,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 913,
                                                            end: 916,
                                                            as_str(): "foo",
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
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 916,
                                                            end: 918,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 919,
                                                                    end: 921,
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
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 922,
                                                                                end: 926,
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
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 926,
                                                    end: 927,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 904,
                                        end: 929,
                                        as_str(): "{\n    fn foo() -> bool;\n}",
                                    },
                                },
                                trait_defs_opt: None,
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
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 931,
                                            end: 933,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 934,
                                            end: 938,
                                            as_str(): "void",
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
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 938,
                                            end: 940,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 941,
                                        end: 944,
                                        as_str(): "{\n}",
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
