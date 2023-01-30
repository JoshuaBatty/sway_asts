Some(
    LexedProgram {
        kind: Contract,
        root: LexedModule {
            tree: Module {
                kind: Contract {
                    contract_token: ContractToken {
                        span: Span {
                            src (ptr): 0x00007fb10cf456b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                            ),
                            start: 0,
                            end: 8,
                            as_str(): "contract",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb10cf456b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
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
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 11,
                                        end: 12,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10cf456b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                        ),
                                                        start: 13,
                                                        end: 20,
                                                        as_str(): "storage",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                            ),
                                                                            start: 21,
                                                                            end: 25,
                                                                            as_str(): "read",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                            ),
                                                                            start: 25,
                                                                            end: 26,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb10cf456b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                        ),
                                                                        start: 27,
                                                                        end: 32,
                                                                        as_str(): "write",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 20,
                                                            end: 33,
                                                            as_str(): "(read, write)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 12,
                                        end: 34,
                                        as_str(): "[storage(read, write)]",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 36,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10cf456b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                        ),
                                                        start: 37,
                                                        end: 40,
                                                        as_str(): "doc",
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
                                                                        src (ptr): 0x00007fb10cf456b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                        ),
                                                                        start: 41,
                                                                        end: 45,
                                                                        as_str(): "test",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 40,
                                                            end: 46,
                                                            as_str(): "(test)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 36,
                                        end: 47,
                                        as_str(): "[doc(test)]",
                                    },
                                },
                            },
                        ],
                        value: Abi(
                            ItemAbi {
                                abi_token: AbiToken {
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 51,
                                        as_str(): "abi",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 52,
                                        end: 59,
                                        as_str(): "GoodAbi",
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
                                                                src (ptr): 0x00007fb10cf456b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
                                                                as_str(): "#",
                                                            },
                                                        },
                                                        attribute: SquareBrackets {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    Attribute {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 68,
                                                                                end: 75,
                                                                                as_str(): "storage",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        args: Some(
                                                                            Parens {
                                                                                inner: Punctuated {
                                                                                    value_separator_pairs: [
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                                    ),
                                                                                                    start: 76,
                                                                                                    end: 80,
                                                                                                    as_str(): "read",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                                    ),
                                                                                                    start: 80,
                                                                                                    end: 81,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    final_value_opt: Some(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                                ),
                                                                                                start: 82,
                                                                                                end: 87,
                                                                                                as_str(): "write",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                    ),
                                                                                    start: 75,
                                                                                    end: 88,
                                                                                    as_str(): "(read, write)",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb10cf456b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                ),
                                                                start: 67,
                                                                end: 89,
                                                                as_str(): "[storage(read, write)]",
                                                            },
                                                        },
                                                    },
                                                    AttributeDecl {
                                                        hash_token: HashToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10cf456b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 95,
                                                                as_str(): "#",
                                                            },
                                                        },
                                                        attribute: SquareBrackets {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    Attribute {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 96,
                                                                                end: 99,
                                                                                as_str(): "doc",
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
                                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                                ),
                                                                                                start: 100,
                                                                                                end: 104,
                                                                                                as_str(): "test",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                    ),
                                                                                    start: 99,
                                                                                    end: 105,
                                                                                    as_str(): "(test)",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb10cf456b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                ),
                                                                start: 95,
                                                                end: 106,
                                                                as_str(): "[doc(test)]",
                                                            },
                                                        },
                                                    },
                                                ],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 113,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 123,
                                                            as_str(): "good_func",
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
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 123,
                                                            end: 125,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 128,
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
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 129,
                                                                                end: 133,
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
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 134,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                        (
                                            Annotated {
                                                attribute_list: [
                                                    AttributeDecl {
                                                        hash_token: HashToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10cf456b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                ),
                                                                start: 140,
                                                                end: 141,
                                                                as_str(): "#",
                                                            },
                                                        },
                                                        attribute: SquareBrackets {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    Attribute {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 142,
                                                                                end: 150,
                                                                                as_str(): "bad_attr",
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
                                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                                ),
                                                                                                start: 151,
                                                                                                end: 155,
                                                                                                as_str(): "blah",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                    ),
                                                                                    start: 150,
                                                                                    end: 156,
                                                                                    as_str(): "(blah)",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb10cf456b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                ),
                                                                start: 141,
                                                                end: 157,
                                                                as_str(): "[bad_attr(blah)]",
                                                            },
                                                        },
                                                    },
                                                ],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 162,
                                                            end: 164,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 165,
                                                            end: 173,
                                                            as_str(): "bad_func",
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
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 173,
                                                            end: 175,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                    ),
                                                                    start: 176,
                                                                    end: 178,
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
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 179,
                                                                                end: 183,
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
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 183,
                                                    end: 184,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 60,
                                        end: 186,
                                        as_str(): "{\n    #[storage(read, write)]\n    #[doc(test)]\n    fn good_func() -> bool;\n\n    #[bad_attr(blah)]\n    fn bad_func() -> bool;\n}",
                                    },
                                },
                                abi_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 188,
                                        end: 192,
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
                                                        src (ptr): 0x00007fb10cf456b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                        ),
                                                        start: 193,
                                                        end: 200,
                                                        as_str(): "GoodAbi",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fb10cf456b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                ),
                                                start: 201,
                                                end: 204,
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
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 205,
                                                    end: 213,
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
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 220,
                                                            end: 221,
                                                            as_str(): "#",
                                                        },
                                                    },
                                                    attribute: SquareBrackets {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Attribute {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                            ),
                                                                            start: 222,
                                                                            end: 229,
                                                                            as_str(): "storage",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    args: Some(
                                                                        Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                                ),
                                                                                                start: 230,
                                                                                                end: 234,
                                                                                                as_str(): "read",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                                ),
                                                                                                start: 234,
                                                                                                end: 235,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                            ),
                                                                                            start: 236,
                                                                                            end: 241,
                                                                                            as_str(): "write",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 229,
                                                                                end: 242,
                                                                                as_str(): "(read, write)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 243,
                                                            as_str(): "[storage(read, write)]",
                                                        },
                                                    },
                                                },
                                                AttributeDecl {
                                                    hash_token: HashToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 248,
                                                            end: 249,
                                                            as_str(): "#",
                                                        },
                                                    },
                                                    attribute: SquareBrackets {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Attribute {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                            ),
                                                                            start: 250,
                                                                            end: 253,
                                                                            as_str(): "doc",
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
                                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                            ),
                                                                                            start: 254,
                                                                                            end: 258,
                                                                                            as_str(): "Test",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 253,
                                                                                end: 259,
                                                                                as_str(): "(Test)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 249,
                                                            end: 260,
                                                            as_str(): "[doc(Test)]",
                                                        },
                                                    },
                                                },
                                            ],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 267,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 268,
                                                            end: 277,
                                                            as_str(): "good_func",
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
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 277,
                                                            end: 279,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                    ),
                                                                    start: 280,
                                                                    end: 282,
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
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 283,
                                                                                end: 287,
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
                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                            ),
                                                                            start: 298,
                                                                            end: 302,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb10cf456b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                        ),
                                                        start: 288,
                                                        end: 308,
                                                        as_str(): "{\n        true\n    }",
                                                    },
                                                },
                                            },
                                        },
                                        Annotated {
                                            attribute_list: [
                                                AttributeDecl {
                                                    hash_token: HashToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 314,
                                                            end: 315,
                                                            as_str(): "#",
                                                        },
                                                    },
                                                    attribute: SquareBrackets {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Attribute {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                            ),
                                                                            start: 316,
                                                                            end: 324,
                                                                            as_str(): "bad_attr",
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
                                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                            ),
                                                                                            start: 325,
                                                                                            end: 329,
                                                                                            as_str(): "blah",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 324,
                                                                                end: 330,
                                                                                as_str(): "(blah)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 315,
                                                            end: 331,
                                                            as_str(): "[bad_attr(blah)]",
                                                        },
                                                    },
                                                },
                                            ],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 336,
                                                            end: 338,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 339,
                                                            end: 347,
                                                            as_str(): "bad_func",
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
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 349,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10cf456b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                    ),
                                                                    start: 350,
                                                                    end: 352,
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
                                                                                src (ptr): 0x00007fb10cf456b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                                ),
                                                                                start: 353,
                                                                                end: 357,
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
                                                                            src (ptr): 0x00007fb10cf456b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                            ),
                                                                            start: 368,
                                                                            end: 372,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb10cf456b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                        ),
                                                        start: 358,
                                                        end: 378,
                                                        as_str(): "{\n        true\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 214,
                                        end: 380,
                                        as_str(): "{\n    #[storage(read, write)]\n    #[doc(Test)]\n    fn good_func() -> bool {\n        true\n    }\n\n    #[bad_attr(blah)]\n    fn bad_func() -> bool {\n        true\n    }\n}",
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
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 382,
                                        end: 383,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb10cf456b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                        ),
                                                        start: 384,
                                                        end: 392,
                                                        as_str(): "bad_attr",
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
                                                                        src (ptr): 0x00007fb10cf456b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                                        ),
                                                                        start: 393,
                                                                        end: 397,
                                                                        as_str(): "blah",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb10cf456b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                            ),
                                                            start: 392,
                                                            end: 398,
                                                            as_str(): "(blah)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 383,
                                        end: 399,
                                        as_str(): "[bad_attr(blah)]",
                                    },
                                },
                            },
                        ],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 400,
                                        end: 406,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 407,
                                        end: 416,
                                        as_str(): "BadStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 417,
                                        end: 419,
                                        as_str(): "{}",
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
