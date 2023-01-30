Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe06737e0f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe06737e0f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe06737e0f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 13,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06737e0f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                        ),
                                        start: 14,
                                        end: 22,
                                        as_str(): "SomeEnum",
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
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 29,
                                                                end: 30,
                                                                as_str(): "B",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 30,
                                                                end: 31,
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
                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                            ),
                                                                            start: 32,
                                                                            end: 36,
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
                                                        src (ptr): 0x00007fe06737e0f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                        ),
                                                        start: 36,
                                                        end: 37,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06737e0f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                        ),
                                        start: 23,
                                        end: 39,
                                        as_str(): "{\n    B: bool,\n}",
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
                                            src (ptr): 0x00007fe06737e0f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                            ),
                                            start: 41,
                                            end: 43,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06737e0f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                            ),
                                            start: 44,
                                            end: 48,
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
                                            src (ptr): 0x00007fe06737e0f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 50,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06737e0f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                    ),
                                                    start: 51,
                                                    end: 53,
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
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
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
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 64,
                                                            end: 67,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 71,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 74,
                                                                end: 80,
                                                                as_str(): "script",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 81,
                                                            end: 82,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 83,
                                                                    end: 84,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 84,
                                                            end: 85,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 93,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 94,
                                                                    end: 97,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 100,
                                                                end: 108,
                                                                as_str(): "contract",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 109,
                                                            end: 110,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 111,
                                                                    end: 112,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 113,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 118,
                                                            end: 121,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 122,
                                                                    end: 125,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 128,
                                                                end: 137,
                                                                as_str(): "predicate",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 138,
                                                            end: 139,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 140,
                                                                    end: 141,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 141,
                                                            end: 142,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 147,
                                                            end: 150,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 151,
                                                                    end: 154,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 164,
                                                                as_str(): "library",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 165,
                                                            end: 166,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 167,
                                                                    end: 168,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 168,
                                                            end: 169,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 174,
                                                            end: 177,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 178,
                                                                    end: 181,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 184,
                                                                end: 187,
                                                                as_str(): "dep",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 188,
                                                            end: 189,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 190,
                                                                    end: 191,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 191,
                                                            end: 192,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 197,
                                                            end: 200,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 201,
                                                                    end: 204,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 207,
                                                                end: 210,
                                                                as_str(): "pub",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 211,
                                                            end: 212,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 213,
                                                                    end: 214,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 214,
                                                            end: 215,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 220,
                                                            end: 223,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 224,
                                                                    end: 227,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 230,
                                                                end: 233,
                                                                as_str(): "use",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 234,
                                                            end: 235,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 236,
                                                                    end: 237,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 237,
                                                            end: 238,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 243,
                                                            end: 246,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 247,
                                                                    end: 250,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 253,
                                                                end: 255,
                                                                as_str(): "as",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 256,
                                                            end: 257,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 258,
                                                                    end: 259,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 260,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 268,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 269,
                                                                    end: 272,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 275,
                                                                end: 281,
                                                                as_str(): "struct",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 282,
                                                            end: 283,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 284,
                                                                    end: 285,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 285,
                                                            end: 286,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 291,
                                                            end: 294,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 295,
                                                                    end: 298,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 301,
                                                                end: 305,
                                                                as_str(): "enum",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 306,
                                                            end: 307,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 308,
                                                                    end: 309,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 309,
                                                            end: 310,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 315,
                                                            end: 318,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 319,
                                                                    end: 322,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 325,
                                                                end: 329,
                                                                as_str(): "self",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 330,
                                                            end: 331,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 332,
                                                                    end: 333,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 333,
                                                            end: 334,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 339,
                                                            end: 342,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 343,
                                                                    end: 346,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 349,
                                                                end: 351,
                                                                as_str(): "fn",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 352,
                                                            end: 353,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 354,
                                                                    end: 355,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 355,
                                                            end: 356,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 361,
                                                            end: 364,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 365,
                                                                    end: 368,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 371,
                                                                end: 376,
                                                                as_str(): "trait",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 377,
                                                            end: 378,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 379,
                                                                    end: 380,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 380,
                                                            end: 381,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 386,
                                                            end: 389,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 390,
                                                                    end: 393,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 396,
                                                                end: 400,
                                                                as_str(): "impl",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 401,
                                                            end: 402,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 403,
                                                                    end: 404,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 404,
                                                            end: 405,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 410,
                                                            end: 413,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 414,
                                                                    end: 417,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 420,
                                                                end: 423,
                                                                as_str(): "for",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 424,
                                                            end: 425,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 426,
                                                                    end: 427,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 427,
                                                            end: 428,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 433,
                                                            end: 436,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 437,
                                                                    end: 440,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 443,
                                                                end: 446,
                                                                as_str(): "abi",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 447,
                                                            end: 448,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 449,
                                                                    end: 450,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 450,
                                                            end: 451,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 456,
                                                            end: 459,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 460,
                                                                    end: 463,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 466,
                                                                end: 471,
                                                                as_str(): "const",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 472,
                                                            end: 473,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 474,
                                                                    end: 475,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 475,
                                                            end: 476,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 481,
                                                            end: 484,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 485,
                                                                    end: 488,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 491,
                                                                end: 498,
                                                                as_str(): "storage",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 499,
                                                            end: 500,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 501,
                                                                    end: 502,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 502,
                                                            end: 503,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 508,
                                                            end: 511,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 512,
                                                                    end: 515,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 518,
                                                                end: 521,
                                                                as_str(): "str",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 522,
                                                            end: 523,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 524,
                                                                    end: 525,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 525,
                                                            end: 526,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 531,
                                                            end: 534,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 535,
                                                                    end: 538,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 541,
                                                                end: 544,
                                                                as_str(): "asm",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 545,
                                                            end: 546,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 547,
                                                                    end: 548,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 548,
                                                            end: 549,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 554,
                                                            end: 557,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 558,
                                                                    end: 561,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 564,
                                                                end: 570,
                                                                as_str(): "return",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 571,
                                                            end: 572,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 573,
                                                                    end: 574,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 574,
                                                            end: 575,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 580,
                                                            end: 583,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 584,
                                                                    end: 587,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 590,
                                                                end: 592,
                                                                as_str(): "if",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 593,
                                                            end: 594,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 595,
                                                                    end: 596,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 596,
                                                            end: 597,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 602,
                                                            end: 605,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 606,
                                                                    end: 609,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 612,
                                                                end: 616,
                                                                as_str(): "else",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 617,
                                                            end: 618,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 619,
                                                                    end: 620,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 620,
                                                            end: 621,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 626,
                                                            end: 629,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 630,
                                                                    end: 633,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 636,
                                                                end: 641,
                                                                as_str(): "match",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 642,
                                                            end: 643,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 644,
                                                                    end: 645,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 645,
                                                            end: 646,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 651,
                                                            end: 654,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 655,
                                                                    end: 658,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 661,
                                                                end: 664,
                                                                as_str(): "mut",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 665,
                                                            end: 666,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 667,
                                                                    end: 668,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 668,
                                                            end: 669,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 674,
                                                            end: 677,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 678,
                                                                    end: 681,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 684,
                                                                end: 687,
                                                                as_str(): "let",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 688,
                                                            end: 689,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 690,
                                                                    end: 691,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 691,
                                                            end: 692,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 697,
                                                            end: 700,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 701,
                                                                    end: 704,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 707,
                                                                end: 712,
                                                                as_str(): "while",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 713,
                                                            end: 714,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 715,
                                                                    end: 716,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 716,
                                                            end: 717,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 722,
                                                            end: 725,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 726,
                                                                    end: 729,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 732,
                                                                end: 737,
                                                                as_str(): "where",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 738,
                                                            end: 739,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 740,
                                                                    end: 741,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 741,
                                                            end: 742,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 747,
                                                            end: 750,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 751,
                                                                    end: 754,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 757,
                                                                end: 760,
                                                                as_str(): "ref",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 761,
                                                            end: 762,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 763,
                                                                    end: 764,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 764,
                                                            end: 765,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 770,
                                                            end: 773,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 774,
                                                                    end: 777,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 780,
                                                                end: 785,
                                                                as_str(): "deref",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 786,
                                                            end: 787,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 788,
                                                                    end: 789,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 789,
                                                            end: 790,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 795,
                                                            end: 798,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 799,
                                                                    end: 802,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 805,
                                                                end: 809,
                                                                as_str(): "true",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 810,
                                                            end: 811,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 812,
                                                                    end: 813,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 813,
                                                            end: 814,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 819,
                                                            end: 822,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 823,
                                                                    end: 826,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 829,
                                                                end: 834,
                                                                as_str(): "false",
                                                            },
                                                            is_raw_ident: true,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 835,
                                                            end: 836,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                    ),
                                                                    start: 837,
                                                                    end: 838,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 838,
                                                            end: 839,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 845,
                                                            end: 848,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 849,
                                                                end: 850,
                                                                as_str(): "e",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 851,
                                                            end: 852,
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
                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                            ),
                                                                            start: 853,
                                                                            end: 861,
                                                                            as_str(): "SomeEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                ),
                                                                                start: 861,
                                                                                end: 863,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                    ),
                                                                                    start: 863,
                                                                                    end: 864,
                                                                                    as_str(): "B",
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
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                    ),
                                                                                    start: 865,
                                                                                    end: 870,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 864,
                                                                end: 871,
                                                                as_str(): "(false)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 871,
                                                            end: 872,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 877,
                                                            end: 880,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 881,
                                                                end: 882,
                                                                as_str(): "v",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 883,
                                                            end: 884,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 885,
                                                                end: 890,
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
                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                            ),
                                                                            start: 891,
                                                                            end: 892,
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
                                                        branches: Braces {
                                                            inner: [
                                                                MatchBranch {
                                                                    pattern: Constructor {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                        ),
                                                                                        start: 903,
                                                                                        end: 911,
                                                                                        as_str(): "SomeEnum",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 911,
                                                                                            end: 913,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                ),
                                                                                                start: 913,
                                                                                                end: 914,
                                                                                                as_str(): "B",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 915,
                                                                                                    end: 919,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                ),
                                                                                start: 914,
                                                                                end: 920,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                            ),
                                                                            start: 921,
                                                                            end: 923,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 938,
                                                                                                    end: 939,
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
                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                ),
                                                                                start: 924,
                                                                                end: 949,
                                                                                as_str(): "{\n            1\n        }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                    ),
                                                                                    start: 949,
                                                                                    end: 950,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Constructor {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06737e0f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                        ),
                                                                                        start: 959,
                                                                                        end: 967,
                                                                                        as_str(): "SomeEnum",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                            ),
                                                                                            start: 967,
                                                                                            end: 969,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                ),
                                                                                                start: 969,
                                                                                                end: 970,
                                                                                                as_str(): "B",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 971,
                                                                                                    end: 976,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                ),
                                                                                start: 970,
                                                                                end: 977,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06737e0f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                            ),
                                                                            start: 978,
                                                                            end: 980,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                                    ),
                                                                                                    start: 995,
                                                                                                    end: 996,
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
                                                                                src (ptr): 0x00007fe06737e0f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                ),
                                                                                start: 981,
                                                                                end: 1006,
                                                                                as_str(): "{\n            0\n        }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06737e0f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                                    ),
                                                                                    start: 1006,
                                                                                    end: 1007,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe06737e0f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                                ),
                                                                start: 893,
                                                                end: 1013,
                                                                as_str(): "{\n        SomeEnum::B(true) => {\n            1\n        },\n        SomeEnum::B(false) => {\n            0\n        },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 1013,
                                                            end: 1014,
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
                                                            src (ptr): 0x00007fe06737e0f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                                            ),
                                                            start: 1020,
                                                            end: 1021,
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
                                        src (ptr): 0x00007fe06737e0f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXba9Z/raw_identifiers/src/main.sw",
                                        ),
                                        start: 58,
                                        end: 1023,
                                        as_str(): "{\n    let mut r#script = 0;\n    let mut r#contract = 0;\n    let mut r#predicate = 0;\n    let mut r#library = 0;\n    let mut r#dep = 0;\n    let mut r#pub = 0;\n    let mut r#use = 0;\n    let mut r#as = 0;\n    let mut r#struct = 0;\n    let mut r#enum = 0;\n    let mut r#self = 0;\n    let mut r#fn = 0;\n    let mut r#trait = 0;\n    let mut r#impl = 0;\n    let mut r#for = 0;\n    let mut r#abi = 0;\n    let mut r#const = 0;\n    let mut r#storage = 0;\n    let mut r#str = 0;\n    let mut r#asm = 0;\n    let mut r#return = 0;\n    let mut r#if = 0;\n    let mut r#else = 0;\n    let mut r#match = 0;\n    let mut r#mut = 0;\n    let mut r#let = 0;\n    let mut r#while = 0;\n    let mut r#where = 0;\n    let mut r#ref = 0;\n    let mut r#deref = 0;\n    let mut r#true = 0;\n    let mut r#false = 0;\n\n    let e = SomeEnum::B(false);\n    let v = match e {\n        SomeEnum::B(true) => {\n            1\n        },\n        SomeEnum::B(false) => {\n            0\n        },\n    };\n\n    0\n}",
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
