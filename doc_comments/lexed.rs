Some(
    LexedProgram {
        kind: Contract,
        root: LexedModule {
            tree: Module {
                kind: Contract {
                    contract_token: ContractToken {
                        span: Span {
                            src (ptr): 0x00007fb101445150,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                            ),
                            start: 0,
                            end: 8,
                            as_str(): "contract",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb101445150,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                        ),
                        start: 8,
                        end: 9,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 11,
                                        end: 60,
                                        as_str(): "/// Enum representing either a number or a string",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 11,
                                                        end: 60,
                                                        as_str(): "/// Enum representing either a number or a string",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 14,
                                                                        end: 60,
                                                                        as_str(): " Enum representing either a number or a string",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 14,
                                                            end: 60,
                                                            as_str(): " Enum representing either a number or a string",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 11,
                                        end: 60,
                                        as_str(): "/// Enum representing either a number or a string",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 64,
                                        as_str(): "///",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 61,
                                                        end: 64,
                                                        as_str(): "///",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 64,
                                                                        end: 64,
                                                                        as_str(): "",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 64,
                                                            end: 64,
                                                            as_str(): "",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 64,
                                        as_str(): "///",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 79,
                                        as_str(): "/// # Examples",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 65,
                                                        end: 79,
                                                        as_str(): "/// # Examples",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 68,
                                                                        end: 79,
                                                                        as_str(): " # Examples",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 68,
                                                            end: 79,
                                                            as_str(): " # Examples",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 79,
                                        as_str(): "/// # Examples",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 83,
                                        as_str(): "///",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 80,
                                                        end: 83,
                                                        as_str(): "///",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 83,
                                                                        end: 83,
                                                                        as_str(): "",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 83,
                                                            end: 83,
                                                            as_str(): "",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 83,
                                        as_str(): "///",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 84,
                                        end: 116,
                                        as_str(): "/// `NumberOrString::Number(42)`",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 84,
                                                        end: 116,
                                                        as_str(): "/// `NumberOrString::Number(42)`",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 87,
                                                                        end: 116,
                                                                        as_str(): " `NumberOrString::Number(42)`",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 87,
                                                            end: 116,
                                                            as_str(): " `NumberOrString::Number(42)`",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 84,
                                        end: 116,
                                        as_str(): "/// `NumberOrString::Number(42)`",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 117,
                                        end: 152,
                                        as_str(): "/// `NumberOrString::String(\"foo\")`",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 117,
                                                        end: 152,
                                                        as_str(): "/// `NumberOrString::String(\"foo\")`",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 120,
                                                                        end: 152,
                                                                        as_str(): " `NumberOrString::String(\"foo\")`",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 120,
                                                            end: 152,
                                                            as_str(): " `NumberOrString::String(\"foo\")`",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 117,
                                        end: 152,
                                        as_str(): "/// `NumberOrString::String(\"foo\")`",
                                    },
                                },
                            },
                        ],
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 153,
                                        end: 157,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 172,
                                        as_str(): "NumberOrString",
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
                                                    attribute_list: [
                                                        AttributeDecl {
                                                            hash_token: HashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 179,
                                                                    end: 223,
                                                                    as_str(): "/// The `Number` variant in `NumberOrString`",
                                                                },
                                                            },
                                                            attribute: SquareBrackets {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Attribute {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "doc-comment",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101445150,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                    ),
                                                                                    start: 179,
                                                                                    end: 223,
                                                                                    as_str(): "/// The `Number` variant in `NumberOrString`",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            args: Some(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                    ),
                                                                                                    start: 182,
                                                                                                    end: 223,
                                                                                                    as_str(): " The `Number` variant in `NumberOrString`",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101445150,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                        ),
                                                                                        start: 182,
                                                                                        end: 223,
                                                                                        as_str(): " The `Number` variant in `NumberOrString`",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 179,
                                                                    end: 223,
                                                                    as_str(): "/// The `Number` variant in `NumberOrString`",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 228,
                                                                end: 234,
                                                                as_str(): "Number",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 234,
                                                                end: 235,
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
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 236,
                                                                            end: 239,
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
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 239,
                                                        end: 240,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [
                                                        AttributeDecl {
                                                            hash_token: HashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 245,
                                                                    end: 289,
                                                                    as_str(): "/// The `String` variant in `NumberOrString`",
                                                                },
                                                            },
                                                            attribute: SquareBrackets {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Attribute {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "doc-comment",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101445150,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                    ),
                                                                                    start: 245,
                                                                                    end: 289,
                                                                                    as_str(): "/// The `String` variant in `NumberOrString`",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            args: Some(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                    ),
                                                                                                    start: 248,
                                                                                                    end: 289,
                                                                                                    as_str(): " The `String` variant in `NumberOrString`",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101445150,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                        ),
                                                                                        start: 248,
                                                                                        end: 289,
                                                                                        as_str(): " The `String` variant in `NumberOrString`",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 245,
                                                                    end: 289,
                                                                    as_str(): "/// The `String` variant in `NumberOrString`",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 294,
                                                                end: 300,
                                                                as_str(): "String",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 300,
                                                                end: 301,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 302,
                                                                    end: 305,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101445150,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                ),
                                                                                start: 306,
                                                                                end: 307,
                                                                                as_str(): "4",
                                                                            },
                                                                            parsed: 4,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 305,
                                                                    end: 308,
                                                                    as_str(): "[4]",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 308,
                                                        end: 309,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 173,
                                        end: 311,
                                        as_str(): "{\n    /// The `Number` variant in `NumberOrString`\n    Number: u64,\n    /// The `String` variant in `NumberOrString`\n    String: str[4],\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 313,
                                        end: 332,
                                        as_str(): "/// Struct holding:",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 313,
                                                        end: 332,
                                                        as_str(): "/// Struct holding:",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 316,
                                                                        end: 332,
                                                                        as_str(): " Struct holding:",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 316,
                                                            end: 332,
                                                            as_str(): " Struct holding:",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 313,
                                        end: 332,
                                        as_str(): "/// Struct holding:",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 333,
                                        end: 336,
                                        as_str(): "///",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 333,
                                                        end: 336,
                                                        as_str(): "///",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 336,
                                                                        end: 336,
                                                                        as_str(): "",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 336,
                                                            end: 336,
                                                            as_str(): "",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 333,
                                        end: 336,
                                        as_str(): "///",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 337,
                                        end: 378,
                                        as_str(): "/// 1. A `value` of type `NumberOrString`",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 337,
                                                        end: 378,
                                                        as_str(): "/// 1. A `value` of type `NumberOrString`",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 340,
                                                                        end: 378,
                                                                        as_str(): " 1. A `value` of type `NumberOrString`",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 378,
                                                            as_str(): " 1. A `value` of type `NumberOrString`",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 337,
                                        end: 378,
                                        as_str(): "/// 1. A `value` of type `NumberOrString`",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 379,
                                        end: 412,
                                        as_str(): "/// 2. An `address` of type `u64`",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 379,
                                                        end: 412,
                                                        as_str(): "/// 2. An `address` of type `u64`",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 382,
                                                                        end: 412,
                                                                        as_str(): " 2. An `address` of type `u64`",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 382,
                                                            end: 412,
                                                            as_str(): " 2. An `address` of type `u64`",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 379,
                                        end: 412,
                                        as_str(): "/// 2. An `address` of type `u64`",
                                    },
                                },
                            },
                        ],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 413,
                                        end: 419,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 420,
                                        end: 424,
                                        as_str(): "Data",
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
                                                    attribute_list: [
                                                        AttributeDecl {
                                                            hash_token: HashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 431,
                                                                    end: 462,
                                                                    as_str(): "/// The `value` field in `Data`",
                                                                },
                                                            },
                                                            attribute: SquareBrackets {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Attribute {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "doc-comment",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101445150,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                    ),
                                                                                    start: 431,
                                                                                    end: 462,
                                                                                    as_str(): "/// The `value` field in `Data`",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            args: Some(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                    ),
                                                                                                    start: 434,
                                                                                                    end: 462,
                                                                                                    as_str(): " The `value` field in `Data`",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101445150,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                        ),
                                                                                        start: 434,
                                                                                        end: 462,
                                                                                        as_str(): " The `value` field in `Data`",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 431,
                                                                    end: 462,
                                                                    as_str(): "/// The `value` field in `Data`",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 467,
                                                                end: 472,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 472,
                                                                end: 473,
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
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 474,
                                                                            end: 488,
                                                                            as_str(): "NumberOrString",
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
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 488,
                                                        end: 489,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [
                                                        AttributeDecl {
                                                            hash_token: HashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 494,
                                                                    end: 527,
                                                                    as_str(): "/// The `address` field in `Data`",
                                                                },
                                                            },
                                                            attribute: SquareBrackets {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Attribute {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "doc-comment",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101445150,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                    ),
                                                                                    start: 494,
                                                                                    end: 527,
                                                                                    as_str(): "/// The `address` field in `Data`",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            args: Some(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                    ),
                                                                                                    start: 497,
                                                                                                    end: 527,
                                                                                                    as_str(): " The `address` field in `Data`",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101445150,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                        ),
                                                                                        start: 497,
                                                                                        end: 527,
                                                                                        as_str(): " The `address` field in `Data`",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 494,
                                                                    end: 527,
                                                                    as_str(): "/// The `address` field in `Data`",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 532,
                                                                end: 539,
                                                                as_str(): "address",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 539,
                                                                end: 540,
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
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 541,
                                                                            end: 544,
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
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 544,
                                                        end: 545,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 425,
                                        end: 547,
                                        as_str(): "{\n    /// The `value` field in `Data`\n    value: NumberOrString,\n    /// The `address` field in `Data`\n    address: u64,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 549,
                                        end: 577,
                                        as_str(): "/// This is the `FooABI` abi",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 549,
                                                        end: 577,
                                                        as_str(): "/// This is the `FooABI` abi",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 552,
                                                                        end: 577,
                                                                        as_str(): " This is the `FooABI` abi",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 552,
                                                            end: 577,
                                                            as_str(): " This is the `FooABI` abi",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 549,
                                        end: 577,
                                        as_str(): "/// This is the `FooABI` abi",
                                    },
                                },
                            },
                        ],
                        value: Abi(
                            ItemAbi {
                                abi_token: AbiToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 578,
                                        end: 581,
                                        as_str(): "abi",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 582,
                                        end: 588,
                                        as_str(): "FooABI",
                                    },
                                    is_raw_ident: false,
                                },
                                abi_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [
                                                    AttributeDecl {
                                                        hash_token: HashToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 595,
                                                                end: 644,
                                                                as_str(): "/// This is the `main` method on the `FooABI` abi",
                                                            },
                                                        },
                                                        attribute: SquareBrackets {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    Attribute {
                                                                        name: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "doc-comment",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101445150,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                ),
                                                                                start: 595,
                                                                                end: 644,
                                                                                as_str(): "/// This is the `main` method on the `FooABI` abi",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        args: Some(
                                                                            Parens {
                                                                                inner: Punctuated {
                                                                                    value_separator_pairs: [],
                                                                                    final_value_opt: Some(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb101445150,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                ),
                                                                                                start: 598,
                                                                                                end: 644,
                                                                                                as_str(): " This is the `main` method on the `FooABI` abi",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101445150,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                    ),
                                                                                    start: 598,
                                                                                    end: 644,
                                                                                    as_str(): " This is the `main` method on the `FooABI` abi",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 595,
                                                                end: 644,
                                                                as_str(): "/// This is the `main` method on the `FooABI` abi",
                                                            },
                                                        },
                                                    },
                                                ],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 649,
                                                            end: 651,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 656,
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
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 656,
                                                            end: 658,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 659,
                                                                    end: 661,
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
                                                                                src (ptr): 0x00007fb101445150,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                ),
                                                                                start: 662,
                                                                                end: 665,
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
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb101445150,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                    ),
                                                    start: 665,
                                                    end: 666,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 589,
                                        end: 668,
                                        as_str(): "{\n    /// This is the `main` method on the `FooABI` abi\n    fn main() -> u64;\n}",
                                    },
                                },
                                abi_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 670,
                                        end: 705,
                                        as_str(): "/// Storage fields for the contract",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 670,
                                                        end: 705,
                                                        as_str(): "/// Storage fields for the contract",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 673,
                                                                        end: 705,
                                                                        as_str(): " Storage fields for the contract",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 673,
                                                            end: 705,
                                                            as_str(): " Storage fields for the contract",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 670,
                                        end: 705,
                                        as_str(): "/// Storage fields for the contract",
                                    },
                                },
                            },
                        ],
                        value: Storage(
                            ItemStorage {
                                storage_token: StorageToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 706,
                                        end: 713,
                                        as_str(): "storage",
                                    },
                                },
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
                                                Annotated {
                                                    attribute_list: [
                                                        AttributeDecl {
                                                            hash_token: HashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 720,
                                                                    end: 737,
                                                                    as_str(): "/// A `u64` field",
                                                                },
                                                            },
                                                            attribute: SquareBrackets {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Attribute {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "doc-comment",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101445150,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                    ),
                                                                                    start: 720,
                                                                                    end: 737,
                                                                                    as_str(): "/// A `u64` field",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            args: Some(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                    ),
                                                                                                    start: 723,
                                                                                                    end: 737,
                                                                                                    as_str(): " A `u64` field",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101445150,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                        ),
                                                                                        start: 723,
                                                                                        end: 737,
                                                                                        as_str(): " A `u64` field",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 720,
                                                                    end: 737,
                                                                    as_str(): "/// A `u64` field",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    value: StorageField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 742,
                                                                end: 749,
                                                                as_str(): "field_a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 749,
                                                                end: 750,
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
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 751,
                                                                            end: 754,
                                                                            as_str(): "u64",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 755,
                                                                end: 756,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 757,
                                                                        end: 758,
                                                                        as_str(): "0",
                                                                    },
                                                                    parsed: 0,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 758,
                                                        end: 759,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [
                                                        AttributeDecl {
                                                            hash_token: HashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 764,
                                                                    end: 782,
                                                                    as_str(): "/// An `str` field",
                                                                },
                                                            },
                                                            attribute: SquareBrackets {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Attribute {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "doc-comment",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101445150,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                    ),
                                                                                    start: 764,
                                                                                    end: 782,
                                                                                    as_str(): "/// An `str` field",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            args: Some(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                    ),
                                                                                                    start: 767,
                                                                                                    end: 782,
                                                                                                    as_str(): " An `str` field",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101445150,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                        ),
                                                                                        start: 767,
                                                                                        end: 782,
                                                                                        as_str(): " An `str` field",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 764,
                                                                    end: 782,
                                                                    as_str(): "/// An `str` field",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    value: StorageField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 787,
                                                                end: 794,
                                                                as_str(): "field_b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 794,
                                                                end: 795,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 796,
                                                                    end: 799,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101445150,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                ),
                                                                                start: 800,
                                                                                end: 801,
                                                                                as_str(): "4",
                                                                            },
                                                                            parsed: 4,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 799,
                                                                    end: 802,
                                                                    as_str(): "[4]",
                                                                },
                                                            },
                                                        },
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101445150,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                ),
                                                                start: 803,
                                                                end: 804,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Literal(
                                                            String(
                                                                LitString {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 805,
                                                                        end: 811,
                                                                        as_str(): "\"aaaa\"",
                                                                    },
                                                                    parsed: "aaaa",
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 811,
                                                        end: 812,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 714,
                                        end: 814,
                                        as_str(): "{\n    /// A `u64` field\n    field_a: u64 = 0,\n    /// An `str` field\n    field_b: str[4] = \"aaaa\",\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 816,
                                        end: 858,
                                        as_str(): "/// The implementation of the `FooABI` abi",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: Some(
                                                        "doc-comment",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 816,
                                                        end: 858,
                                                        as_str(): "/// The implementation of the `FooABI` abi",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101445150,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                        ),
                                                                        start: 819,
                                                                        end: 858,
                                                                        as_str(): " The implementation of the `FooABI` abi",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 819,
                                                            end: 858,
                                                            as_str(): " The implementation of the `FooABI` abi",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 816,
                                        end: 858,
                                        as_str(): "/// The implementation of the `FooABI` abi",
                                    },
                                },
                            },
                        ],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 859,
                                        end: 863,
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
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 864,
                                                        end: 870,
                                                        as_str(): "FooABI",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fb101445150,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                ),
                                                start: 871,
                                                end: 874,
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
                                                    src (ptr): 0x00007fb101445150,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                    ),
                                                    start: 875,
                                                    end: 883,
                                                    as_str(): "Contract",
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
                                            attribute_list: [
                                                AttributeDecl {
                                                    hash_token: HashToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 890,
                                                            end: 937,
                                                            as_str(): "/// The main function that does all the things!",
                                                        },
                                                    },
                                                    attribute: SquareBrackets {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Attribute {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "doc-comment",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 890,
                                                                            end: 937,
                                                                            as_str(): "/// The main function that does all the things!",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    args: Some(
                                                                        Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101445150,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                            ),
                                                                                            start: 893,
                                                                                            end: 937,
                                                                                            as_str(): " The main function that does all the things!",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101445150,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                ),
                                                                                start: 893,
                                                                                end: 937,
                                                                                as_str(): " The main function that does all the things!",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 890,
                                                            end: 937,
                                                            as_str(): "/// The main function that does all the things!",
                                                        },
                                                    },
                                                },
                                            ],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 942,
                                                            end: 944,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 945,
                                                            end: 949,
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
                                                            src (ptr): 0x00007fb101445150,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                            ),
                                                            start: 949,
                                                            end: 951,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101445150,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                    ),
                                                                    start: 952,
                                                                    end: 954,
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
                                                                                src (ptr): 0x00007fb101445150,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                ),
                                                                                start: 955,
                                                                                end: 958,
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
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 969,
                                                                            end: 972,
                                                                            as_str(): "let",
                                                                        },
                                                                    },
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: Some(
                                                                            MutToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101445150,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                    ),
                                                                                    start: 973,
                                                                                    end: 976,
                                                                                    as_str(): "mut",
                                                                                },
                                                                            },
                                                                        ),
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101445150,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                ),
                                                                                start: 977,
                                                                                end: 981,
                                                                                as_str(): "data",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    ty_opt: None,
                                                                    eq_token: EqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 982,
                                                                            end: 983,
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
                                                                                        src (ptr): 0x00007fb101445150,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                        ),
                                                                                        start: 984,
                                                                                        end: 988,
                                                                                        as_str(): "Data",
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
                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1003,
                                                                                                    end: 1008,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb101445150,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1008,
                                                                                                            end: 1009,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    FuncApp {
                                                                                                        func: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb101445150,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1010,
                                                                                                                            end: 1024,
                                                                                                                            as_str(): "NumberOrString",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [
                                                                                                                    (
                                                                                                                        DoubleColonToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb101445150,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1024,
                                                                                                                                end: 1026,
                                                                                                                                as_str(): "::",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1026,
                                                                                                                                    end: 1032,
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
                                                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1033,
                                                                                                                                    end: 1035,
                                                                                                                                    as_str(): "20",
                                                                                                                                },
                                                                                                                                parsed: 20,
                                                                                                                                ty_opt: None,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                ),
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb101445150,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1032,
                                                                                                                end: 1036,
                                                                                                                as_str(): "(20)",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb101445150,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                ),
                                                                                                start: 1036,
                                                                                                end: 1037,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    (
                                                                                        ExprStructField {
                                                                                            field_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1050,
                                                                                                    end: 1057,
                                                                                                    as_str(): "address",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb101445150,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1057,
                                                                                                            end: 1058,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb101445150,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1059,
                                                                                                                    end: 1063,
                                                                                                                    as_str(): "1337",
                                                                                                                },
                                                                                                                parsed: 1337,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb101445150,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                                ),
                                                                                                start: 1063,
                                                                                                end: 1064,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101445150,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                ),
                                                                                start: 989,
                                                                                end: 1074,
                                                                                as_str(): "{\n            value: NumberOrString::Number(20),\n            address: 1337,\n        }",
                                                                            },
                                                                        },
                                                                    },
                                                                    semicolon_token: SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 1074,
                                                                            end: 1075,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                            Expr {
                                                                expr: Return {
                                                                    return_token: ReturnToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 1085,
                                                                            end: 1091,
                                                                            as_str(): "return",
                                                                        },
                                                                    },
                                                                    expr_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101445150,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                                        ),
                                                                                        start: 1092,
                                                                                        end: 1094,
                                                                                        as_str(): "20",
                                                                                    },
                                                                                    parsed: 20,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101445150,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                                            ),
                                                                            start: 1094,
                                                                            end: 1095,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ],
                                                        final_expr_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb101445150,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                                        ),
                                                        start: 959,
                                                        end: 1101,
                                                        as_str(): "{\n        let mut data = Data {\n            value: NumberOrString::Number(20),\n            address: 1337,\n        };\n\n        return 20;\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb101445150,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaRYgNz/doc_comments/src/main.sw",
                                        ),
                                        start: 884,
                                        end: 1103,
                                        as_str(): "{\n    /// The main function that does all the things!\n    fn main() -> u64 {\n        let mut data = Data {\n            value: NumberOrString::Number(20),\n            address: 1337,\n        };\n\n        return 20;\n    }\n}",
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
