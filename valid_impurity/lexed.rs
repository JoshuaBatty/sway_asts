Some(
    LexedProgram {
        kind: Contract,
        root: LexedModule {
            tree: Module {
                kind: Contract {
                    contract_token: ContractToken {
                        span: Span {
                            src (ptr): 0x00007f8a11564cc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                            ),
                            start: 0,
                            end: 8,
                            as_str(): "contract",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a11564cc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                        ),
                        start: 8,
                        end: 9,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Abi(
                            ItemAbi {
                                abi_token: AbiToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 11,
                                        end: 14,
                                        as_str(): "abi",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 15,
                                        end: 27,
                                        as_str(): "ImpurityTest",
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
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
                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                ),
                                                                                start: 36,
                                                                                end: 43,
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
                                                                                                    src (ptr): 0x00007f8a11564cc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                                    ),
                                                                                                    start: 44,
                                                                                                    end: 48,
                                                                                                    as_str(): "read",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a11564cc0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                                    ),
                                                                                                    start: 48,
                                                                                                    end: 49,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    final_value_opt: Some(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                                ),
                                                                                                start: 50,
                                                                                                end: 55,
                                                                                                as_str(): "write",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a11564cc0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                    ),
                                                                                    start: 43,
                                                                                    end: 56,
                                                                                    as_str(): "(read, write)",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 57,
                                                                as_str(): "[storage(read, write)]",
                                                            },
                                                        },
                                                    },
                                                ],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 62,
                                                            end: 64,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 65,
                                                            end: 76,
                                                            as_str(): "impure_func",
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
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 76,
                                                            end: 78,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a11564cc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 81,
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
                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                ),
                                                                                start: 82,
                                                                                end: 86,
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
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 86,
                                                    end: 87,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 28,
                                        end: 89,
                                        as_str(): "{\n    #[storage(read, write)]\n    fn impure_func() -> bool;\n}",
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
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 91,
                                        end: 95,
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 96,
                                                        end: 108,
                                                        as_str(): "ImpurityTest",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007f8a11564cc0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                ),
                                                start: 109,
                                                end: 112,
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
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 121,
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
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 129,
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
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 130,
                                                                            end: 137,
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
                                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                                ),
                                                                                                start: 138,
                                                                                                end: 142,
                                                                                                as_str(): "read",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                                ),
                                                                                                start: 142,
                                                                                                end: 143,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                            ),
                                                                                            start: 144,
                                                                                            end: 149,
                                                                                            as_str(): "write",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                ),
                                                                                start: 137,
                                                                                end: 150,
                                                                                as_str(): "(read, write)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 129,
                                                            end: 151,
                                                            as_str(): "[storage(read, write)]",
                                                        },
                                                    },
                                                },
                                            ],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 156,
                                                            end: 158,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 159,
                                                            end: 170,
                                                            as_str(): "impure_func",
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
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 172,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a11564cc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                    ),
                                                                    start: 173,
                                                                    end: 175,
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
                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                ),
                                                                                start: 176,
                                                                                end: 180,
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
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 191,
                                                                            end: 194,
                                                                            as_str(): "let",
                                                                        },
                                                                    },
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                ),
                                                                                start: 195,
                                                                                end: 196,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    ty_opt: None,
                                                                    eq_token: EqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 197,
                                                                            end: 198,
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
                                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                            ),
                                                                                            start: 199,
                                                                                            end: 222,
                                                                                            as_str(): "can_also_read_and_write",
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
                                                                                src (ptr): 0x00007f8a11564cc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                                ),
                                                                                start: 222,
                                                                                end: 224,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    semicolon_token: SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 224,
                                                                            end: 225,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_expr_opt: Some(
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 234,
                                                                            end: 238,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 181,
                                                        end: 244,
                                                        as_str(): "{\n        let a = can_also_read_and_write();\n        true\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 122,
                                        end: 246,
                                        as_str(): "{\n    #[storage(read, write)]\n    fn impure_func() -> bool {\n        let a = can_also_read_and_write();\n        true\n    }\n}",
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
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 250,
                                                        end: 257,
                                                        as_str(): "storage",
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
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 258,
                                                                        end: 262,
                                                                        as_str(): "read",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 257,
                                                            end: 263,
                                                            as_str(): "(read)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 249,
                                        end: 264,
                                        as_str(): "[storage(read)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 265,
                                            end: 267,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 268,
                                            end: 281,
                                            as_str(): "can_read_only",
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
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 281,
                                            end: 283,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 284,
                                                    end: 286,
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 287,
                                                                end: 290,
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
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 297,
                                                            end: 300,
                                                            as_str(): "100",
                                                        },
                                                        parsed: 100,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 291,
                                        end: 302,
                                        as_str(): "{\n    100\n}",
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
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 304,
                                        end: 305,
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 306,
                                                        end: 313,
                                                        as_str(): "storage",
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
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 314,
                                                                        end: 318,
                                                                        as_str(): "read",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 313,
                                                            end: 319,
                                                            as_str(): "(read)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 305,
                                        end: 320,
                                        as_str(): "[storage(read)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 321,
                                            end: 323,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 324,
                                            end: 342,
                                            as_str(): "can_also_read_only",
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
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 342,
                                            end: 344,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 345,
                                                    end: 347,
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 348,
                                                                end: 351,
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
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a11564cc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                    ),
                                                                    start: 358,
                                                                    end: 371,
                                                                    as_str(): "can_read_only",
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 371,
                                                        end: 373,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 352,
                                        end: 375,
                                        as_str(): "{\n    can_read_only()\n}",
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
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 377,
                                        end: 378,
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 379,
                                                        end: 386,
                                                        as_str(): "storage",
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
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 387,
                                                                        end: 392,
                                                                        as_str(): "write",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 386,
                                                            end: 393,
                                                            as_str(): "(write)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 378,
                                        end: 394,
                                        as_str(): "[storage(write)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 395,
                                            end: 397,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 398,
                                            end: 412,
                                            as_str(): "can_write_only",
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
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 412,
                                            end: 414,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 415,
                                                    end: 417,
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 418,
                                                                end: 421,
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
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 428,
                                                            end: 431,
                                                            as_str(): "101",
                                                        },
                                                        parsed: 101,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 422,
                                        end: 433,
                                        as_str(): "{\n    101\n}",
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
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 435,
                                        end: 436,
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 437,
                                                        end: 444,
                                                        as_str(): "storage",
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
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 445,
                                                                        end: 450,
                                                                        as_str(): "write",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 444,
                                                            end: 451,
                                                            as_str(): "(write)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 436,
                                        end: 452,
                                        as_str(): "[storage(write)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 453,
                                            end: 455,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 456,
                                            end: 475,
                                            as_str(): "can_also_write_only",
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
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 475,
                                            end: 477,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 478,
                                                    end: 480,
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 481,
                                                                end: 484,
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
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a11564cc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                    ),
                                                                    start: 491,
                                                                    end: 505,
                                                                    as_str(): "can_write_only",
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 505,
                                                        end: 507,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 485,
                                        end: 509,
                                        as_str(): "{\n    can_write_only()\n}",
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
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 511,
                                        end: 512,
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 513,
                                                        end: 520,
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
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 521,
                                                                            end: 525,
                                                                            as_str(): "read",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 525,
                                                                            end: 526,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 527,
                                                                        end: 532,
                                                                        as_str(): "write",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 520,
                                                            end: 533,
                                                            as_str(): "(read, write)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 512,
                                        end: 534,
                                        as_str(): "[storage(read, write)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 535,
                                            end: 537,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 538,
                                            end: 556,
                                            as_str(): "can_read_and_write",
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
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 556,
                                            end: 558,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 559,
                                                    end: 561,
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 562,
                                                                end: 565,
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
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 572,
                                                            end: 575,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 576,
                                                                end: 577,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 578,
                                                            end: 579,
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
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 580,
                                                                            end: 598,
                                                                            as_str(): "can_also_read_only",
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 598,
                                                                end: 600,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 600,
                                                            end: 601,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 606,
                                                            end: 609,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 610,
                                                                end: 611,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 612,
                                                            end: 613,
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
                                                                            src (ptr): 0x00007f8a11564cc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                            ),
                                                                            start: 614,
                                                                            end: 633,
                                                                            as_str(): "can_also_write_only",
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 633,
                                                                end: 635,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 635,
                                                            end: 636,
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
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 641,
                                                            end: 644,
                                                            as_str(): "102",
                                                        },
                                                        parsed: 102,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 566,
                                        end: 646,
                                        as_str(): "{\n    let a = can_also_read_only();\n    let b = can_also_write_only();\n    102\n}",
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
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 648,
                                        end: 649,
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 650,
                                                        end: 657,
                                                        as_str(): "storage",
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
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 658,
                                                                        end: 662,
                                                                        as_str(): "read",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 657,
                                                            end: 663,
                                                            as_str(): "(read)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 649,
                                        end: 664,
                                        as_str(): "[storage(read)]",
                                    },
                                },
                            },
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 665,
                                        end: 666,
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 667,
                                                        end: 674,
                                                        as_str(): "storage",
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
                                                                        src (ptr): 0x00007f8a11564cc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                        ),
                                                                        start: 675,
                                                                        end: 680,
                                                                        as_str(): "write",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a11564cc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                            ),
                                                            start: 674,
                                                            end: 681,
                                                            as_str(): "(write)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 666,
                                        end: 682,
                                        as_str(): "[storage(write)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 683,
                                            end: 685,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 686,
                                            end: 709,
                                            as_str(): "can_also_read_and_write",
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
                                            src (ptr): 0x00007f8a11564cc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                            ),
                                            start: 709,
                                            end: 711,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a11564cc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                    ),
                                                    start: 712,
                                                    end: 714,
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
                                                                src (ptr): 0x00007f8a11564cc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                ),
                                                                start: 715,
                                                                end: 718,
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
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a11564cc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                                    ),
                                                                    start: 725,
                                                                    end: 743,
                                                                    as_str(): "can_read_and_write",
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
                                                        src (ptr): 0x00007f8a11564cc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                                        ),
                                                        start: 743,
                                                        end: 745,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a11564cc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvfviX/valid_impurity/src/main.sw",
                                        ),
                                        start: 719,
                                        end: 747,
                                        as_str(): "{\n    can_read_and_write()\n}",
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
