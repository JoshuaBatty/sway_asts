Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a28ab8be0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a28ab8be0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
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
                                        src (ptr): 0x00007f8a28ab8be0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 13,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a28ab8be0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                        ),
                                        start: 14,
                                        end: 15,
                                        as_str(): "E",
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
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 23,
                                                                as_str(): "A",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 23,
                                                                end: 24,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a28ab8be0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                    ),
                                                                    start: 25,
                                                                    end: 27,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28ab8be0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                        ),
                                                        start: 27,
                                                        end: 28,
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
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 33,
                                                                end: 34,
                                                                as_str(): "B",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a28ab8be0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                    ),
                                                                    start: 36,
                                                                    end: 38,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28ab8be0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                        ),
                                                        start: 38,
                                                        end: 39,
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
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 44,
                                                                end: 45,
                                                                as_str(): "C",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 45,
                                                                end: 46,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a28ab8be0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                    ),
                                                                    start: 47,
                                                                    end: 49,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a28ab8be0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                        ),
                                                        start: 49,
                                                        end: 50,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a28ab8be0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 52,
                                        as_str(): "{\n    A: (),\n    B: (),\n    C: (),\n}",
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
                                            src (ptr): 0x00007f8a28ab8be0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                            ),
                                            start: 54,
                                            end: 56,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a28ab8be0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                            ),
                                            start: 57,
                                            end: 61,
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
                                            src (ptr): 0x00007f8a28ab8be0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                            ),
                                            start: 61,
                                            end: 63,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a28ab8be0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                    ),
                                                    start: 64,
                                                    end: 66,
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
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 67,
                                                                end: 68,
                                                                as_str(): "E",
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
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a28ab8be0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                ),
                                                                start: 197,
                                                                end: 198,
                                                                as_str(): "E",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    suffix: [
                                                        (
                                                            DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a28ab8be0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                    ),
                                                                    start: 198,
                                                                    end: 200,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a28ab8be0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                                                        ),
                                                                        start: 200,
                                                                        end: 201,
                                                                        as_str(): "C",
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a28ab8be0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR9CSXYS/unit_type_variants/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 203,
                                        as_str(): "{\n    // Expected output is only 8 bytes because all the variants are unit types \n    //\n    //  0000000000000002  # E.tag\n\n    E::C\n}",
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
