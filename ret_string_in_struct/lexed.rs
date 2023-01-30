Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe06dc66160,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe06dc66160,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
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
                                        src (ptr): 0x00007fe06dc66160,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06dc66160,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 23,
                                        as_str(): "Wrapper",
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
                                                                src (ptr): 0x00007fe06dc66160,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                ),
                                                                start: 30,
                                                                end: 34,
                                                                as_str(): "name",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06dc66160,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06dc66160,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 36,
                                                                    end: 39,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06dc66160,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 40,
                                                                                end: 41,
                                                                                as_str(): "9",
                                                                            },
                                                                            parsed: 9,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06dc66160,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 39,
                                                                    end: 42,
                                                                    as_str(): "[9]",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dc66160,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                        ),
                                                        start: 42,
                                                        end: 43,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06dc66160,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                        ),
                                        start: 24,
                                        end: 45,
                                        as_str(): "{\n    name: str[9],\n}",
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
                                            src (ptr): 0x00007fe06dc66160,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                            ),
                                            start: 47,
                                            end: 49,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06dc66160,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                            ),
                                            start: 50,
                                            end: 54,
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
                                            src (ptr): 0x00007fe06dc66160,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                            ),
                                            start: 54,
                                            end: 56,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06dc66160,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                    ),
                                                    start: 57,
                                                    end: 59,
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
                                                                src (ptr): 0x00007fe06dc66160,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                ),
                                                                start: 60,
                                                                end: 67,
                                                                as_str(): "Wrapper",
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
                                            Struct {
                                                path: PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06dc66160,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                ),
                                                                start: 74,
                                                                end: 81,
                                                                as_str(): "Wrapper",
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
                                                                            src (ptr): 0x00007fe06dc66160,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 92,
                                                                            end: 96,
                                                                            as_str(): "name",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    expr_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06dc66160,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 96,
                                                                                    end: 97,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Literal(
                                                                                String(
                                                                                    LitString {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe06dc66160,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 98,
                                                                                            end: 109,
                                                                                            as_str(): "\"fuel-labs\"",
                                                                                        },
                                                                                        parsed: "fuel-labs",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06dc66160,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 109,
                                                                        end: 110,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dc66160,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                        ),
                                                        start: 82,
                                                        end: 116,
                                                        as_str(): "{\n        name: \"fuel-labs\",\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06dc66160,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                        ),
                                        start: 68,
                                        end: 118,
                                        as_str(): "{\n    Wrapper {\n        name: \"fuel-labs\",\n    }\n}",
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
