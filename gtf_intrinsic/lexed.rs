Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0fb225230,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0fb225230,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
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
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 14,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 15,
                                        end: 28,
                                        as_str(): "SOME_TX_FIELD",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 29,
                                        end: 30,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0fb225230,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                ),
                                                start: 31,
                                                end: 35,
                                                as_str(): "0x42",
                                            },
                                            parsed: 66,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 36,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 37,
                                        end: 42,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 62,
                                        as_str(): "SOME_OTHER_TX_FIELD",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 63,
                                        end: 64,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0fb225230,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                ),
                                                start: 65,
                                                end: 69,
                                                as_str(): "0x77",
                                            },
                                            parsed: 119,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 70,
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
                                            src (ptr): 0x00007fe0fb225230,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                            ),
                                            start: 72,
                                            end: 74,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb225230,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                            ),
                                            start: 75,
                                            end: 79,
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
                                            src (ptr): 0x00007fe0fb225230,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                            ),
                                            start: 79,
                                            end: 81,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb225230,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                    ),
                                                    start: 82,
                                                    end: 84,
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
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 85,
                                                                end: 88,
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
                                                            src (ptr): 0x00007fe0fb225230,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                            ),
                                                            start: 186,
                                                            end: 189,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 190,
                                                                end: 199,
                                                                as_str(): "u64_field",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb225230,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                            ),
                                                            start: 200,
                                                            end: 201,
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
                                                                            src (ptr): 0x00007fe0fb225230,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 202,
                                                                            end: 207,
                                                                            as_str(): "__gtf",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: Some(
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb225230,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 207,
                                                                                    end: 209,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            GenericArgs {
                                                                                parameters: AngleBrackets {
                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb225230,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 209,
                                                                                            end: 210,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                    },
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb225230,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 210,
                                                                                                                end: 213,
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
                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb225230,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 213,
                                                                                            end: 214,
                                                                                            as_str(): ">",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
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
                                                                                        src (ptr): 0x00007fe0fb225230,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 215,
                                                                                        end: 216,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb225230,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 216,
                                                                                end: 217,
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
                                                                                        src (ptr): 0x00007fe0fb225230,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 218,
                                                                                        end: 231,
                                                                                        as_str(): "SOME_TX_FIELD",
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
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 214,
                                                                end: 232,
                                                                as_str(): "(1, SOME_TX_FIELD)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb225230,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
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
                                                            src (ptr): 0x00007fe0fb225230,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                            ),
                                                            start: 238,
                                                            end: 241,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 242,
                                                                end: 252,
                                                                as_str(): "b256_field",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb225230,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 254,
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
                                                                            src (ptr): 0x00007fe0fb225230,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 255,
                                                                            end: 260,
                                                                            as_str(): "__gtf",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: Some(
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb225230,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 260,
                                                                                    end: 262,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            GenericArgs {
                                                                                parameters: AngleBrackets {
                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb225230,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 262,
                                                                                            end: 263,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                    },
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb225230,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                                                ),
                                                                                                                start: 263,
                                                                                                                end: 267,
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
                                                                                    },
                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb225230,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 267,
                                                                                            end: 268,
                                                                                            as_str(): ">",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
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
                                                                                        src (ptr): 0x00007fe0fb225230,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 269,
                                                                                        end: 270,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb225230,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 270,
                                                                                end: 271,
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
                                                                                        src (ptr): 0x00007fe0fb225230,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 272,
                                                                                        end: 291,
                                                                                        as_str(): "SOME_OTHER_TX_FIELD",
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
                                                                src (ptr): 0x00007fe0fb225230,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                                ),
                                                                start: 268,
                                                                end: 292,
                                                                as_str(): "(2, SOME_OTHER_TX_FIELD)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb225230,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                            ),
                                                            start: 292,
                                                            end: 293,
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
                                                            src (ptr): 0x00007fe0fb225230,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 299,
                                                            as_str(): "0",
                                                        },
                                                        parsed: 0,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fb225230,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFLpWp7/gtf_intrinsic/src/main.sw",
                                        ),
                                        start: 89,
                                        end: 301,
                                        as_str(): "{\n    // Test expected to compile but revert because `fuel-core` does not support `gtf` yet.\n    let u64_field = __gtf::<u64>(1, SOME_TX_FIELD);\n    let b256_field = __gtf::<b256>(2, SOME_OTHER_TX_FIELD);\n    0\n}",
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
