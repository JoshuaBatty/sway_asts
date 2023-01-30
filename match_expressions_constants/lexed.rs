Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0b1cf1900,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0b1cf1900,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
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
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 14,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 15,
                                        end: 23,
                                        as_str(): "NUMBER_1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 23,
                                                end: 24,
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
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 25,
                                                            end: 28,
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
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
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
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 31,
                                                end: 32,
                                                as_str(): "7",
                                            },
                                            parsed: 7,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
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
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 34,
                                        end: 39,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 40,
                                        end: 48,
                                        as_str(): "NUMBER_2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 48,
                                                end: 49,
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
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 50,
                                                            end: 53,
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
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 54,
                                        end: 55,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 56,
                                                end: 58,
                                                as_str(): "14",
                                            },
                                            parsed: 14,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 58,
                                        end: 59,
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
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 60,
                                        end: 65,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 66,
                                        end: 74,
                                        as_str(): "NUMBER_3",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 74,
                                                end: 75,
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
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
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
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 81,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 82,
                                                end: 83,
                                                as_str(): "5",
                                            },
                                            parsed: 5,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 83,
                                        end: 84,
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
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 86,
                                        end: 91,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 96,
                                        as_str(): "TRUE",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 96,
                                                end: 97,
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
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 102,
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
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 104,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Bool(
                                        LitBool {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 105,
                                                end: 109,
                                                as_str(): "true",
                                            },
                                            kind: True,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 109,
                                        end: 110,
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
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 111,
                                        end: 116,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 117,
                                        end: 122,
                                        as_str(): "FALSE",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 122,
                                                end: 123,
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
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 124,
                                                            end: 128,
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
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 129,
                                        end: 130,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Bool(
                                        LitBool {
                                            span: Span {
                                                src (ptr): 0x00007fe0b1cf1900,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                ),
                                                start: 131,
                                                end: 136,
                                                as_str(): "false",
                                            },
                                            kind: False,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 136,
                                        end: 137,
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
                                            src (ptr): 0x00007fe0b1cf1900,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                            ),
                                            start: 139,
                                            end: 141,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0b1cf1900,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                            ),
                                            start: 142,
                                            end: 146,
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
                                            src (ptr): 0x00007fe0b1cf1900,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                            ),
                                            start: 146,
                                            end: 148,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0b1cf1900,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                    ),
                                                    start: 149,
                                                    end: 151,
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
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 152,
                                                                end: 155,
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
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 162,
                                                            end: 165,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 166,
                                                                end: 167,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 168,
                                                            end: 169,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b1cf1900,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                    ),
                                                                    start: 170,
                                                                    end: 171,
                                                                    as_str(): "5",
                                                                },
                                                                parsed: 5,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 171,
                                                            end: 172,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 178,
                                                            end: 181,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 182,
                                                                end: 183,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 184,
                                                            end: 185,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 191,
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
                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                            ),
                                                                            start: 192,
                                                                            end: 193,
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
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 204,
                                                                                end: 212,
                                                                                as_str(): "NUMBER_1",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                            ),
                                                                            start: 213,
                                                                            end: 215,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Expr {
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b1cf1900,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 216,
                                                                                        end: 217,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 217,
                                                                                end: 218,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 227,
                                                                                end: 235,
                                                                                as_str(): "NUMBER_2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                            ),
                                                                            start: 236,
                                                                            end: 238,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Expr {
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b1cf1900,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 239,
                                                                                        end: 240,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 240,
                                                                                end: 241,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 250,
                                                                                end: 258,
                                                                                as_str(): "NUMBER_3",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 261,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Expr {
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b1cf1900,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 262,
                                                                                        end: 264,
                                                                                        as_str(): "42",
                                                                                    },
                                                                                    parsed: 42,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 264,
                                                                                end: 265,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 274,
                                                                                end: 279,
                                                                                as_str(): "other",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                            ),
                                                                            start: 280,
                                                                            end: 282,
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
                                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 283,
                                                                                            end: 288,
                                                                                            as_str(): "other",
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
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 288,
                                                                                end: 289,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 194,
                                                                end: 295,
                                                                as_str(): "{\n        NUMBER_1 => 1,\n        NUMBER_2 => 1,\n        NUMBER_3 => 42,\n        other => other,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
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
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 302,
                                                            end: 305,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 306,
                                                                end: 307,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 308,
                                                            end: 309,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b1cf1900,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                    ),
                                                                    start: 310,
                                                                    end: 314,
                                                                    as_str(): "true",
                                                                },
                                                                kind: True,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 314,
                                                            end: 315,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 320,
                                                            end: 323,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 324,
                                                                end: 325,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 326,
                                                            end: 327,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 328,
                                                                end: 333,
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
                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                            ),
                                                                            start: 334,
                                                                            end: 335,
                                                                            as_str(): "c",
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
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 346,
                                                                                end: 350,
                                                                                as_str(): "TRUE",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                            ),
                                                                            start: 351,
                                                                            end: 353,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Expr {
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b1cf1900,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 354,
                                                                                        end: 356,
                                                                                        as_str(): "42",
                                                                                    },
                                                                                    parsed: 42,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 356,
                                                                                end: 357,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 366,
                                                                                end: 371,
                                                                                as_str(): "FALSE",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                            ),
                                                                            start: 372,
                                                                            end: 374,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Expr {
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b1cf1900,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 375,
                                                                                        end: 376,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 376,
                                                                                end: 377,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                ),
                                                                start: 336,
                                                                end: 383,
                                                                as_str(): "{\n        TRUE => 42,\n        FALSE => 1,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 383,
                                                            end: 384,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 390,
                                                            end: 392,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        LogicalAnd {
                                                            lhs: Equal {
                                                                lhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b1cf1900,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 393,
                                                                                    end: 394,
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
                                                                double_eq_token: DoubleEqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b1cf1900,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                        ),
                                                                        start: 395,
                                                                        end: 397,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                rhs: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 398,
                                                                                end: 400,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            double_ampersand_token: DoubleAmpersandToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b1cf1900,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                    ),
                                                                    start: 401,
                                                                    end: 403,
                                                                    as_str(): "&&",
                                                                },
                                                            },
                                                            rhs: Equal {
                                                                lhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b1cf1900,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 404,
                                                                                    end: 405,
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
                                                                double_eq_token: DoubleEqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b1cf1900,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                        ),
                                                                        start: 406,
                                                                        end: 408,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                rhs: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 409,
                                                                                end: 411,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
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
                                                                                src (ptr): 0x00007fe0b1cf1900,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                ),
                                                                                start: 422,
                                                                                end: 424,
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
                                                            src (ptr): 0x00007fe0b1cf1900,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                            ),
                                                            start: 412,
                                                            end: 430,
                                                            as_str(): "{\n        42\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b1cf1900,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                    ),
                                                                    start: 431,
                                                                    end: 435,
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
                                                                                            src (ptr): 0x00007fe0b1cf1900,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 446,
                                                                                            end: 447,
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
                                                                        src (ptr): 0x00007fe0b1cf1900,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                                                        ),
                                                                        start: 436,
                                                                        end: 453,
                                                                        as_str(): "{\n        1\n    }",
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
                                        src (ptr): 0x00007fe0b1cf1900,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRjTWtwv/match_expressions_constants/src/main.sw",
                                        ),
                                        start: 156,
                                        end: 455,
                                        as_str(): "{\n    let a = 5;\n\n    let b = match a {\n        NUMBER_1 => 1,\n        NUMBER_2 => 1,\n        NUMBER_3 => 42,\n        other => other,\n    };\n\n    let c = true;\n    let d = match c {\n        TRUE => 42,\n        FALSE => 1,\n    };\n\n    if b == 42 && d == 42 {\n        42\n    } else {\n        1\n    }\n}",
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
