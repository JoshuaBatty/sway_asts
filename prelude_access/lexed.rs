Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe062436520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe062436520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
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
                                        src (ptr): 0x00007fe062436520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe062436520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 17,
                                        as_str(): "A",
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
                                                                src (ptr): 0x00007fe062436520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                                                ),
                                                                start: 24,
                                                                end: 28,
                                                                as_str(): "addr",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe062436520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                                                ),
                                                                start: 28,
                                                                end: 29,
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
                                                                            src (ptr): 0x00007fe062436520,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                                                            ),
                                                                            start: 30,
                                                                            end: 37,
                                                                            as_str(): "Address",
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
                                                        src (ptr): 0x00007fe062436520,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                                        ),
                                                        start: 37,
                                                        end: 38,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe062436520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 40,
                                        as_str(): "{\n    addr: Address,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fe062436520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                        ),
                                        start: 74,
                                        end: 77,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe062436520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 81,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe062436520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 83,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe062436520,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                                ),
                                                start: 83,
                                                end: 90,
                                                as_str(): "address",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe062436520,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                                ),
                                                start: 90,
                                                end: 92,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe062436520,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                                    ),
                                                    start: 92,
                                                    end: 99,
                                                    as_str(): "Address",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe062436520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                        ),
                                        start: 99,
                                        end: 100,
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
                                            src (ptr): 0x00007fe062436520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                            ),
                                            start: 102,
                                            end: 104,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe062436520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                            ),
                                            start: 105,
                                            end: 109,
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
                                            src (ptr): 0x00007fe062436520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                            ),
                                            start: 109,
                                            end: 111,
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
                                        src (ptr): 0x00007fe062436520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                                        ),
                                        start: 112,
                                        end: 115,
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
