Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb14c154030,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 17,
                                        as_str(): "S",
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 24,
                                                                end: 27,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 27,
                                                                end: 28,
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
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 29,
                                                                            end: 32,
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
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 32,
                                                        end: 33,
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 38,
                                                                end: 41,
                                                                as_str(): "bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 41,
                                                                end: 42,
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
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 43,
                                                                            end: 46,
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
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 46,
                                                        end: 47,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 49,
                                        as_str(): "{\n    foo: u64,\n    bar: u64,\n}",
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
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 51,
                                            end: 53,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 54,
                                            end: 58,
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
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 58,
                                            end: 60,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 63,
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 68,
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
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 78,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 79,
                                                                end: 80,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 80,
                                                                    end: 81,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Array(
                                                                SquareBrackets {
                                                                    inner: TyArrayDescriptor {
                                                                        ty: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 83,
                                                                                            end: 87,
                                                                                            as_str(): "bool",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                            },
                                                                        ),
                                                                        semicolon_token: SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 87,
                                                                                end: 88,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 93,
                                                                                        end: 94,
                                                                                        as_str(): "5",
                                                                                    },
                                                                                    parsed: 5,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 82,
                                                                        end: 95,
                                                                        as_str(): "[bool;\n    5]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 96,
                                                            end: 97,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [
                                                                        (
                                                                            Literal(
                                                                                Bool(
                                                                                    LitBool {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 99,
                                                                                            end: 103,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                        kind: True,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 104,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Literal(
                                                                                Bool(
                                                                                    LitBool {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 105,
                                                                                            end: 109,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                        kind: True,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 109,
                                                                                    end: 110,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Literal(
                                                                                Bool(
                                                                                    LitBool {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 111,
                                                                                            end: 115,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                        kind: True,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 115,
                                                                                    end: 116,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Literal(
                                                                                Bool(
                                                                                    LitBool {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 117,
                                                                                            end: 122,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                        kind: False,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 122,
                                                                                    end: 123,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    final_value_opt: Some(
                                                                        Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 124,
                                                                                        end: 128,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 98,
                                                                end: 129,
                                                                as_str(): "[true, true, true, false, true]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 129,
                                                            end: 130,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 138,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 139,
                                                                end: 140,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 140,
                                                                    end: 141,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Array(
                                                                SquareBrackets {
                                                                    inner: TyArrayDescriptor {
                                                                        ty: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 143,
                                                                                            end: 146,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                            },
                                                                        ),
                                                                        semicolon_token: SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 146,
                                                                                end: 147,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 152,
                                                                                        end: 154,
                                                                                        as_str(): "10",
                                                                                    },
                                                                                    parsed: 10,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 142,
                                                                        end: 155,
                                                                        as_str(): "[u32;\n    10]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 156,
                                                            end: 157,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Repeat {
                                                                value: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 159,
                                                                                end: 160,
                                                                                as_str(): "3",
                                                                            },
                                                                            parsed: 3,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                semicolon_token: SemicolonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 160,
                                                                        end: 161,
                                                                        as_str(): ";",
                                                                    },
                                                                },
                                                                length: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 166,
                                                                                end: 168,
                                                                                as_str(): "10",
                                                                            },
                                                                            parsed: 10,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 158,
                                                                end: 169,
                                                                as_str(): "[3;\n    10]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 170,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 178,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 180,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 181,
                                                            end: 182,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [
                                                                        (
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 184,
                                                                                            end: 188,
                                                                                            as_str(): "0x01",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 188,
                                                                                    end: 189,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 190,
                                                                                            end: 194,
                                                                                            as_str(): "0x02",
                                                                                        },
                                                                                        parsed: 2,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 194,
                                                                                    end: 195,
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
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 196,
                                                                                        end: 200,
                                                                                        as_str(): "0x03",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 183,
                                                                end: 201,
                                                                as_str(): "[0x01, 0x02, 0x03]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 201,
                                                            end: 202,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 207,
                                                            end: 210,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 211,
                                                                end: 212,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 213,
                                                            end: 214,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Repeat {
                                                                value: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 216,
                                                                                end: 217,
                                                                                as_str(): "0",
                                                                            },
                                                                            parsed: 0,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                semicolon_token: SemicolonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 217,
                                                                        end: 218,
                                                                        as_str(): ";",
                                                                    },
                                                                },
                                                                length: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 223,
                                                                                end: 225,
                                                                                as_str(): "10",
                                                                            },
                                                                            parsed: 10,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 215,
                                                                end: 226,
                                                                as_str(): "[0;\n    10]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 226,
                                                            end: 227,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 232,
                                                            end: 235,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 236,
                                                                end: 237,
                                                                as_str(): "e",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 237,
                                                                    end: 238,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Array(
                                                                SquareBrackets {
                                                                    inner: TyArrayDescriptor {
                                                                        ty: Array(
                                                                            SquareBrackets {
                                                                                inner: TyArrayDescriptor {
                                                                                    ty: Path(
                                                                                        PathType {
                                                                                            root_opt: None,
                                                                                            prefix: PathTypeSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 241,
                                                                                                        end: 244,
                                                                                                        as_str(): "u64",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [],
                                                                                        },
                                                                                    ),
                                                                                    semicolon_token: SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 244,
                                                                                            end: 245,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                    length: Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 250,
                                                                                                    end: 251,
                                                                                                    as_str(): "4",
                                                                                                },
                                                                                                parsed: 4,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 240,
                                                                                    end: 252,
                                                                                    as_str(): "[u64;\n    4]",
                                                                                },
                                                                            },
                                                                        ),
                                                                        semicolon_token: SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 252,
                                                                                end: 253,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 258,
                                                                                        end: 259,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 239,
                                                                        end: 260,
                                                                        as_str(): "[[u64;\n    4];\n    2]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 261,
                                                            end: 262,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [
                                                                        (
                                                                            Array(
                                                                                SquareBrackets {
                                                                                    inner: Sequence(
                                                                                        Punctuated {
                                                                                            value_separator_pairs: [
                                                                                                (
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 265,
                                                                                                                    end: 266,
                                                                                                                    as_str(): "1",
                                                                                                                },
                                                                                                                parsed: 1,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 266,
                                                                                                            end: 267,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 268,
                                                                                                                    end: 269,
                                                                                                                    as_str(): "2",
                                                                                                                },
                                                                                                                parsed: 2,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 269,
                                                                                                            end: 270,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 271,
                                                                                                                    end: 272,
                                                                                                                    as_str(): "3",
                                                                                                                },
                                                                                                                parsed: 3,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 272,
                                                                                                            end: 273,
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
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 274,
                                                                                                                end: 275,
                                                                                                                as_str(): "4",
                                                                                                            },
                                                                                                            parsed: 4,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 264,
                                                                                        end: 276,
                                                                                        as_str(): "[1, 2, 3, 4]",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 276,
                                                                                    end: 277,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    final_value_opt: Some(
                                                                        Array(
                                                                            SquareBrackets {
                                                                                inner: Sequence(
                                                                                    Punctuated {
                                                                                        value_separator_pairs: [
                                                                                            (
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 279,
                                                                                                                end: 280,
                                                                                                                as_str(): "5",
                                                                                                            },
                                                                                                            parsed: 5,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 280,
                                                                                                        end: 281,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 282,
                                                                                                                end: 283,
                                                                                                                as_str(): "6",
                                                                                                            },
                                                                                                            parsed: 6,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 283,
                                                                                                        end: 284,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            (
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 285,
                                                                                                                end: 286,
                                                                                                                as_str(): "7",
                                                                                                            },
                                                                                                            parsed: 7,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 286,
                                                                                                        end: 287,
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
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 288,
                                                                                                            end: 289,
                                                                                                            as_str(): "8",
                                                                                                        },
                                                                                                        parsed: 8,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 278,
                                                                                    end: 290,
                                                                                    as_str(): "[5, 6, 7, 8]",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 263,
                                                                end: 291,
                                                                as_str(): "[[1, 2, 3, 4], [5, 6, 7, 8]]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 291,
                                                            end: 292,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 333,
                                                            end: 336,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 337,
                                                                end: 338,
                                                                as_str(): "g",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 339,
                                                            end: 340,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [
                                                                        (
                                                                            Struct {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 342,
                                                                                                end: 343,
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
                                                                                        value_separator_pairs: [
                                                                                            (
                                                                                                ExprStructField {
                                                                                                    field_name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 354,
                                                                                                            end: 357,
                                                                                                            as_str(): "foo",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    expr_opt: Some(
                                                                                                        (
                                                                                                            ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 357,
                                                                                                                    end: 358,
                                                                                                                    as_str(): ":",
                                                                                                                },
                                                                                                            },
                                                                                                            Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 359,
                                                                                                                            end: 361,
                                                                                                                            as_str(): "10",
                                                                                                                        },
                                                                                                                        parsed: 10,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 361,
                                                                                                        end: 362,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: Some(
                                                                                            ExprStructField {
                                                                                                field_name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 363,
                                                                                                        end: 366,
                                                                                                        as_str(): "bar",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                expr_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 366,
                                                                                                                end: 367,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 368,
                                                                                                                        end: 370,
                                                                                                                        as_str(): "20",
                                                                                                                    },
                                                                                                                    parsed: 20,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 344,
                                                                                        end: 376,
                                                                                        as_str(): "{\n        foo: 10, bar: 20\n    }",
                                                                                    },
                                                                                },
                                                                            },
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 376,
                                                                                    end: 377,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    final_value_opt: Some(
                                                                        Struct {
                                                                            path: PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 382,
                                                                                            end: 383,
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
                                                                                    value_separator_pairs: [
                                                                                        (
                                                                                            ExprStructField {
                                                                                                field_name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 394,
                                                                                                        end: 397,
                                                                                                        as_str(): "foo",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                expr_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 397,
                                                                                                                end: 398,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 399,
                                                                                                                        end: 400,
                                                                                                                        as_str(): "1",
                                                                                                                    },
                                                                                                                    parsed: 1,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                            CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 400,
                                                                                                    end: 401,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    final_value_opt: Some(
                                                                                        ExprStructField {
                                                                                            field_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 402,
                                                                                                    end: 405,
                                                                                                    as_str(): "bar",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 405,
                                                                                                            end: 406,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 407,
                                                                                                                    end: 408,
                                                                                                                    as_str(): "2",
                                                                                                                },
                                                                                                                parsed: 2,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 384,
                                                                                    end: 414,
                                                                                    as_str(): "{\n        foo: 1, bar: 2\n    }",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 341,
                                                                end: 420,
                                                                as_str(): "[S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 420,
                                                            end: 421,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 426,
                                                            end: 429,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 430,
                                                                end: 431,
                                                                as_str(): "h",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 432,
                                                            end: 433,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Index {
                                                        target: FuncApp {
                                                            func: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 434,
                                                                                end: 435,
                                                                                as_str(): "i",
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
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 435,
                                                                    end: 437,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        },
                                                        arg: SquareBrackets {
                                                            inner: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 438,
                                                                            end: 439,
                                                                            as_str(): "2",
                                                                        },
                                                                        parsed: 2,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 437,
                                                                end: 440,
                                                                as_str(): "[2]",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 440,
                                                            end: 441,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            LogicalAnd {
                                                lhs: LogicalAnd {
                                                    lhs: LogicalAnd {
                                                        lhs: LogicalAnd {
                                                            lhs: Equal {
                                                                lhs: Index {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 447,
                                                                                        end: 448,
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
                                                                    arg: SquareBrackets {
                                                                        inner: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
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
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 448,
                                                                            end: 451,
                                                                            as_str(): "[0]",
                                                                        },
                                                                    },
                                                                },
                                                                double_eq_token: DoubleEqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 452,
                                                                        end: 454,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                rhs: Index {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 455,
                                                                                        end: 456,
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
                                                                    arg: SquareBrackets {
                                                                        inner: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 457,
                                                                                        end: 458,
                                                                                        as_str(): "9",
                                                                                    },
                                                                                    parsed: 9,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 456,
                                                                            end: 459,
                                                                            as_str(): "[9]",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            double_ampersand_token: DoubleAmpersandToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 460,
                                                                    end: 462,
                                                                    as_str(): "&&",
                                                                },
                                                            },
                                                            rhs: Equal {
                                                                lhs: Add {
                                                                    lhs: Index {
                                                                        target: Index {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 463,
                                                                                                end: 464,
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
                                                                            arg: SquareBrackets {
                                                                                inner: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 465,
                                                                                                end: 466,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                            parsed: 0,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 464,
                                                                                    end: 467,
                                                                                    as_str(): "[0]",
                                                                                },
                                                                            },
                                                                        },
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 468,
                                                                                            end: 469,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 467,
                                                                                end: 470,
                                                                                as_str(): "[1]",
                                                                            },
                                                                        },
                                                                    },
                                                                    add_token: AddToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 471,
                                                                            end: 472,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                    rhs: Index {
                                                                        target: Index {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 473,
                                                                                                end: 474,
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
                                                                            arg: SquareBrackets {
                                                                                inner: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 475,
                                                                                                end: 476,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                            parsed: 1,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 474,
                                                                                    end: 477,
                                                                                    as_str(): "[1]",
                                                                                },
                                                                            },
                                                                        },
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 478,
                                                                                            end: 479,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                        parsed: 2,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 477,
                                                                                end: 480,
                                                                                as_str(): "[2]",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                double_eq_token: DoubleEqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 481,
                                                                        end: 483,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                rhs: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 484,
                                                                                end: 485,
                                                                                as_str(): "9",
                                                                            },
                                                                            parsed: 9,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                        double_ampersand_token: DoubleAmpersandToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 486,
                                                                end: 488,
                                                                as_str(): "&&",
                                                            },
                                                        },
                                                        rhs: Equal {
                                                            lhs: Add {
                                                                lhs: FieldProjection {
                                                                    target: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 489,
                                                                                            end: 490,
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
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 491,
                                                                                            end: 492,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                        parsed: 0,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 490,
                                                                                end: 493,
                                                                                as_str(): "[0]",
                                                                            },
                                                                        },
                                                                    },
                                                                    dot_token: DotToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 493,
                                                                            end: 494,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 494,
                                                                            end: 497,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                add_token: AddToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 498,
                                                                        end: 499,
                                                                        as_str(): "+",
                                                                    },
                                                                },
                                                                rhs: FieldProjection {
                                                                    target: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 500,
                                                                                            end: 501,
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
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 502,
                                                                                            end: 503,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 501,
                                                                                end: 504,
                                                                                as_str(): "[1]",
                                                                            },
                                                                        },
                                                                    },
                                                                    dot_token: DotToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 504,
                                                                            end: 505,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 505,
                                                                            end: 508,
                                                                            as_str(): "bar",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                            },
                                                            double_eq_token: DoubleEqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 509,
                                                                    end: 511,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            rhs: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 512,
                                                                            end: 514,
                                                                            as_str(): "12",
                                                                        },
                                                                        parsed: 12,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    double_ampersand_token: DoubleAmpersandToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 515,
                                                            end: 517,
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
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 518,
                                                                            end: 519,
                                                                            as_str(): "j",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 520,
                                                                                        end: 521,
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
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 519,
                                                                end: 522,
                                                                as_str(): "(g)",
                                                            },
                                                        },
                                                    },
                                                },
                                                double_ampersand_token: DoubleAmpersandToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 523,
                                                        end: 525,
                                                        as_str(): "&&",
                                                    },
                                                },
                                                rhs: Literal(
                                                    Bool(
                                                        LitBool {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 552,
                                                                end: 556,
                                                                as_str(): "true",
                                                            },
                                                            kind: True,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 558,
                                        as_str(): "{\n    let a: [bool;\n    5] = [true, true, true, false, true];\n    let b: [u32;\n    10] = [3;\n    10];\n    let c = [0x01, 0x02, 0x03];\n    let d = [0;\n    10];\n    let e: [[u64;\n    4];\n    2] = [[1, 2, 3, 4], [5, 6, 7, 8]];\n    //let f: [u64; 1 + 1] = [0, 0];\n    let g = [S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ];\n    let h = i()[2];\n\n    b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true\n}",
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
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 560,
                                            end: 562,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 563,
                                            end: 564,
                                            as_str(): "i",
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
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 564,
                                            end: 566,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 567,
                                                    end: 569,
                                                    as_str(): "->",
                                                },
                                            },
                                            Array(
                                                SquareBrackets {
                                                    inner: TyArrayDescriptor {
                                                        ty: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 571,
                                                                            end: 574,
                                                                            as_str(): "u64",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        semicolon_token: SemicolonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 574,
                                                                end: 575,
                                                                as_str(): ";",
                                                            },
                                                        },
                                                        length: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 576,
                                                                        end: 577,
                                                                        as_str(): "4",
                                                                    },
                                                                    parsed: 4,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 570,
                                                        end: 578,
                                                        as_str(): "[u64;\n4]",
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
                                            Array(
                                                SquareBrackets {
                                                    inner: Sequence(
                                                        Punctuated {
                                                            value_separator_pairs: [
                                                                (
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 586,
                                                                                    end: 587,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 587,
                                                                            end: 588,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 589,
                                                                                    end: 590,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 590,
                                                                            end: 591,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 592,
                                                                                    end: 593,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 593,
                                                                            end: 594,
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
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 595,
                                                                                end: 596,
                                                                                as_str(): "3",
                                                                            },
                                                                            parsed: 3,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 585,
                                                        end: 597,
                                                        as_str(): "[0, 1, 2, 3]",
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 579,
                                        end: 599,
                                        as_str(): "{\n    [0, 1, 2, 3]\n}",
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
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 601,
                                            end: 603,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 604,
                                            end: 605,
                                            as_str(): "j",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 606,
                                                                    end: 613,
                                                                    as_str(): "ary_arg",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 613,
                                                                end: 614,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Array(
                                                            SquareBrackets {
                                                                inner: TyArrayDescriptor {
                                                                    ty: Path(
                                                                        PathType {
                                                                            root_opt: None,
                                                                            prefix: PathTypeSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 616,
                                                                                        end: 617,
                                                                                        as_str(): "S",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                        },
                                                                    ),
                                                                    semicolon_token: SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 617,
                                                                            end: 618,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                    length: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 619,
                                                                                    end: 620,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 615,
                                                                    end: 621,
                                                                    as_str(): "[S;\n2]",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 605,
                                            end: 622,
                                            as_str(): "(ary_arg: [S;\n2])",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 623,
                                                    end: 625,
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 626,
                                                                end: 630,
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
                                            Equal {
                                                lhs: Add {
                                                    lhs: FieldProjection {
                                                        target: Index {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 637,
                                                                                end: 644,
                                                                                as_str(): "ary_arg",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                            ),
                                                            arg: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 645,
                                                                                end: 646,
                                                                                as_str(): "0",
                                                                            },
                                                                            parsed: 0,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 644,
                                                                    end: 647,
                                                                    as_str(): "[0]",
                                                                },
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 647,
                                                                end: 648,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 648,
                                                                end: 651,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    add_token: AddToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 653,
                                                            as_str(): "+",
                                                        },
                                                    },
                                                    rhs: FieldProjection {
                                                        target: Index {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 654,
                                                                                end: 661,
                                                                                as_str(): "ary_arg",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                            ),
                                                            arg: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 662,
                                                                                end: 663,
                                                                                as_str(): "1",
                                                                            },
                                                                            parsed: 1,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 661,
                                                                    end: 664,
                                                                    as_str(): "[1]",
                                                                },
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 664,
                                                                end: 665,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 665,
                                                                end: 668,
                                                                as_str(): "bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                },
                                                double_eq_token: DoubleEqToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 669,
                                                        end: 671,
                                                        as_str(): "==",
                                                    },
                                                },
                                                rhs: Literal(
                                                    Int(
                                                        LitInt {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 672,
                                                                end: 674,
                                                                as_str(): "12",
                                                            },
                                                            parsed: 12,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 631,
                                        end: 676,
                                        as_str(): "{\n    ary_arg[0].foo + ary_arg[1].bar == 12\n}",
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
