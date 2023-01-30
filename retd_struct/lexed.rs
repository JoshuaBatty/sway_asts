Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe05bcc9310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe05bcc9310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
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
                                        src (ptr): 0x00007fe05bcc9310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe05bcc9310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 31,
                                        as_str(): "BiggerThanAWord",
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
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 38,
                                                                end: 45,
                                                                as_str(): "field_1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 45,
                                                                end: 46,
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
                                                                            src (ptr): 0x00007fe05bcc9310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                            ),
                                                                            start: 47,
                                                                            end: 50,
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
                                                        src (ptr): 0x00007fe05bcc9310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                        ),
                                                        start: 50,
                                                        end: 51,
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
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 56,
                                                                end: 63,
                                                                as_str(): "field_2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 64,
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
                                                                            src (ptr): 0x00007fe05bcc9310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                            ),
                                                                            start: 65,
                                                                            end: 69,
                                                                            as_str(): "b256",
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
                                                        src (ptr): 0x00007fe05bcc9310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                        ),
                                                        start: 69,
                                                        end: 70,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05bcc9310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                        ),
                                        start: 32,
                                        end: 72,
                                        as_str(): "{\n    field_1: u64,\n    field_2: b256,\n}",
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
                                            src (ptr): 0x00007fe05bcc9310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                            ),
                                            start: 74,
                                            end: 76,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05bcc9310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                            ),
                                            start: 77,
                                            end: 81,
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
                                            src (ptr): 0x00007fe05bcc9310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 83,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05bcc9310,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                    ),
                                                    start: 84,
                                                    end: 86,
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
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 87,
                                                                end: 102,
                                                                as_str(): "BiggerThanAWord",
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
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 109,
                                                                end: 124,
                                                                as_str(): "BiggerThanAWord",
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
                                                                            src (ptr): 0x00007fe05bcc9310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                            ),
                                                                            start: 135,
                                                                            end: 142,
                                                                            as_str(): "field_1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    expr_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05bcc9310,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 142,
                                                                                    end: 143,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05bcc9310,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 144,
                                                                                            end: 149,
                                                                                            as_str(): "99999",
                                                                                        },
                                                                                        parsed: 99999,
                                                                                        ty_opt: Some(
                                                                                            (
                                                                                                U64,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe05bcc9310,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 149,
                                                                                                    end: 152,
                                                                                                    as_str(): "u64",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05bcc9310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                        ),
                                                                        start: 152,
                                                                        end: 153,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                            (
                                                                ExprStructField {
                                                                    field_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05bcc9310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                            ),
                                                                            start: 162,
                                                                            end: 169,
                                                                            as_str(): "field_2",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    expr_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05bcc9310,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 169,
                                                                                    end: 170,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05bcc9310,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 171,
                                                                                            end: 237,
                                                                                            as_str(): "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                                                                                        },
                                                                                        parsed: 115792089237316195423570985008687907853269984665640564039457584007913129639935,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05bcc9310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                        ),
                                                                        start: 237,
                                                                        end: 238,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe05bcc9310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                        ),
                                                        start: 125,
                                                        end: 244,
                                                        as_str(): "{\n        field_1: 99999u64,\n        field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05bcc9310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 246,
                                        as_str(): "{\n    BiggerThanAWord {\n        field_1: 99999u64,\n        field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,\n    }\n}",
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
